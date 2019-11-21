//! The `drone` module provides an object for launching a Morgan Drone,
//! which is the custodian of any remaining difs in a mint.
//! The Morgan Drone builds and send airdrop transactions,
//! checking requests against a request cap for a given time time_slice
//! and (to come) an IP rate limit.

use bincode::{deserialize, serialize};
use byteorder::{ByteOrder, LittleEndian};
use bytes::{Bytes, BytesMut};
use log::*;
use serde_derive::{Deserialize, Serialize};
use morgan_metricbot::datapoint_info;
use morgan_interface::hash::Hash;
use morgan_interface::message::Message;
use morgan_interface::packet::PACKET_DATA_SIZE;
use morgan_interface::pubkey::Pubkey;
use morgan_interface::signature::{Keypair, KeypairUtil};
use morgan_interface::system_instruction;
use morgan_interface::transaction::Transaction;
use std::io;
use std::io::{Error, ErrorKind};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio;
use tokio::net::TcpListener;
use tokio::prelude::{Future, Read, Sink, Stream, Write};
use tokio_codec::{BytesCodec, Decoder};
use morgan_helper::logHelper::*;

#[macro_export]
macro_rules! socketaddr {
    ($ip:expr, $port:expr) => {
        SocketAddr::from((Ipv4Addr::from($ip), $port))
    };
    ($str:expr) => {{
        let a: SocketAddr = $str.parse().unwrap();
        a
    }};
}

pub const TIME_SLICE: u64 = 60;
pub const REQUEST_CAP: u64 = 100_000_000_000_000;
pub const DRONE_PORT: u16 = 11100;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum DroneRequest {
    GetAirdrop {
        difs: u64,
        to: Pubkey,
        blockhash: Hash,
    },
    GetReputation {
        reputations: u64,
        to: Pubkey,
        blockhash: Hash,
    },
}

pub struct Drone {
    mint_keypair: Keypair,
    ip_cache: Vec<IpAddr>,
    pub time_slice: Duration,
    request_cap: u64,
    pub request_current: u64,
}

impl Drone {
    pub fn new(
        mint_keypair: Keypair,
        time_input: Option<u64>,
        request_cap_input: Option<u64>,
    ) -> Drone {
        let time_slice = match time_input {
            Some(time) => Duration::new(time, 0),
            None => Duration::new(TIME_SLICE, 0),
        };
        let request_cap = match request_cap_input {
            Some(cap) => cap,
            None => REQUEST_CAP,
        };
        Drone {
            mint_keypair,
            ip_cache: Vec::new(),
            time_slice,
            request_cap,
            request_current: 0,
        }
    }

    pub fn check_request_limit(&mut self, request_amount: u64) -> bool {
        (self.request_current + request_amount) <= self.request_cap
    }

    pub fn clear_request_count(&mut self) {
        self.request_current = 0;
    }

    pub fn add_ip_to_cache(&mut self, ip: IpAddr) {
        self.ip_cache.push(ip);
    }

    pub fn clear_ip_cache(&mut self) {
        self.ip_cache.clear();
    }

    pub fn build_airdrop_transaction(
        &mut self,
        req: DroneRequest,
    ) -> Result<Transaction, io::Error> {
        trace!("build_airdrop_transaction: {:?}", req);
        match req {
            DroneRequest::GetAirdrop {
                difs,
                to,
                blockhash,
            } => {
                if self.check_request_limit(difs) {
                    self.request_current += difs;
                    datapoint_info!(
                        "drone-airdrop",
                        ("request_amount", difs, i64),
                        ("request_current", self.request_current, i64)
                    );
                    // info!("{}", Info(format!("Requesting airdrop of {} to {:?}", difs, to).to_string()));
                    println!("{}",
                        printLn(
                            format!("Requesting airdrop of {} to {:?}", difs, to).to_string(),
                            module_path!().to_string()
                        )
                    );
                    let create_instruction = system_instruction::create_user_account(
                        &self.mint_keypair.pubkey(),
                        &to,
                        difs,
                    );
                    let message = Message::new(vec![create_instruction]);
                    Ok(Transaction::new(&[&self.mint_keypair], message, blockhash))
                } else {
                    Err(Error::new(
                        ErrorKind::Other,
                        format!(
                            "token limit reached; req: {} current: {} cap: {}",
                            difs, self.request_current, self.request_cap
                        ),
                    ))
                }
            }
            DroneRequest::GetReputation {
                reputations,
                to,
                blockhash,
            } => {
                if self.check_request_limit(reputations) {
                    self.request_current += reputations;
                    datapoint_info!(
                        "drone-airdrop",
                        ("request_amount", reputations, i64),
                        ("request_current", self.request_current, i64)
                    );
                    // info!("{}", Info(format!("Requesting reputation airdrop of {} to {:?}", reputations, to).to_string()));
                    println!("{}",
                        printLn(
                            format!("Requesting reputation airdrop of {} to {:?}", reputations, to).to_string(),
                            module_path!().to_string()
                        )
                    );
                    let create_instruction = system_instruction::create_user_account_with_reputation(
                        &self.mint_keypair.pubkey(),
                        &to,
                        reputations,
                    );
                    let message = Message::new(vec![create_instruction]);
                    Ok(Transaction::new(&[&self.mint_keypair], message, blockhash))
                } else {
                    Err(Error::new(
                        ErrorKind::Other,
                        format!(
                            "token limit reached; req: {} current: {} cap: {}",
                            reputations, self.request_current, self.request_cap
                        ),
                    ))
                }
            }
        }
    }
    pub fn process_drone_request(&mut self, bytes: &BytesMut) -> Result<Bytes, io::Error> {
        let req: DroneRequest = deserialize(bytes).or_else(|err| {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("deserialize packet in drone: {:?}", err),
            ))
        })?;

        // info!("{}", Info(format!("Airdrop transaction requested...{:?}", req).to_string()));
        println!("{}",
            printLn(
                format!("Airdrop transaction requested...{:?}", req).to_string(),
                module_path!().to_string()
            )
        );
        let res = self.build_airdrop_transaction(req);
        match res {
            Ok(tx) => {
                let response_vec = bincode::serialize(&tx).or_else(|err| {
                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("deserialize packet in drone: {:?}", err),
                    ))
                })?;

                let mut response_vec_with_length = vec![0; 2];
                LittleEndian::write_u16(&mut response_vec_with_length, response_vec.len() as u16);
                response_vec_with_length.extend_from_slice(&response_vec);

                let response_bytes = Bytes::from(response_vec_with_length);
                // info!("{}", Info(format!("Airdrop transaction granted").to_string()));
                println!("{}",
                    printLn(
                        format!("Airdrop transaction granted").to_string(),
                        module_path!().to_string()
                    )
                );
                Ok(response_bytes)
            }
            Err(err) => {
                // warn!("Airdrop transaction failed: {:?}", err);
                println!(
                    "{}",
                    Warn(
                        format!("Airdrop transaction failed: {:?}", err).to_string(),
                        module_path!().to_string())
                );
                Err(err)
            }
        }
    }
}

impl Drop for Drone {
    fn drop(&mut self) {
        morgan_metricbot::flush();
    }
}

pub fn request_airdrop_transaction(
    drone_addr: &SocketAddr,
    id: &Pubkey,
    difs: u64,
    blockhash: Hash,
) -> Result<Transaction, Error> {
    // info!(
    //     "{}",
    //     Info(format!("request_airdrop_transaction: drone_addr={} id={} difs={} blockhash={}",
    //     drone_addr, id, difs, blockhash).to_string())
    // );
    println!("{}",
        printLn(
            format!("request_airdrop_transaction: drone_addr={} id={} difs={} blockhash={}",
                drone_addr, id, difs, blockhash).to_string(),
            module_path!().to_string()
        )
    );
    // TODO: make this async tokio client
    let mut stream = TcpStream::connect_timeout(drone_addr, Duration::new(3, 0))?;
    stream.set_read_timeout(Some(Duration::new(10, 0)))?;
    let req = DroneRequest::GetAirdrop {
        difs,
        blockhash,
        to: *id,
    };
    let req = serialize(&req).expect("serialize drone request");
    stream.write_all(&req)?;

    // Read length of transaction
    let mut buffer = [0; 2];
    stream.read_exact(&mut buffer).or_else(|err| {
        // info!(
        //     "{}",
        //     Info(format!("request_airdrop_transaction: buffer length read_exact error: {:?}",
        //     err).to_string())
        // );
        println!("{}",
            printLn(
                format!("request_airdrop_transaction: buffer length read_exact error: {:?}",
                    err).to_string(),
                module_path!().to_string()
            )
        );
        Err(Error::new(ErrorKind::Other, "Airdrop failed"))
    })?;
    let transaction_length = LittleEndian::read_u16(&buffer) as usize;
    if transaction_length >= PACKET_DATA_SIZE {
        Err(Error::new(
            ErrorKind::Other,
            format!(
                "request_airdrop_transaction: invalid transaction_length from drone: {}",
                transaction_length
            ),
        ))?;
    }

    // Read the transaction
    let mut buffer = Vec::new();
    buffer.resize(transaction_length, 0);
    stream.read_exact(&mut buffer).or_else(|err| {
        // info!(
        //     "{}",
        //     Info(format!("request_airdrop_transaction: buffer read_exact error: {:?}",
        //     err).to_string())
        // );
        println!("{}",
            printLn(
                format!("request_airdrop_transaction: buffer read_exact error: {:?}",
                    err).to_string(),
                module_path!().to_string()
            )
        );
        Err(Error::new(ErrorKind::Other, "Airdrop failed"))
    })?;

    let transaction: Transaction = deserialize(&buffer).or_else(|err| {
        Err(Error::new(
            ErrorKind::Other,
            format!("request_airdrop_transaction deserialize failure: {:?}", err),
        ))
    })?;
    Ok(transaction)
}

pub fn request_reputation_airdrop_transaction(
    drone_addr: &SocketAddr,
    id: &Pubkey,
    reputations: u64,
    blockhash: Hash,
) -> Result<Transaction, Error> {
    // info!(
    //     "{}",
    //     Info(format!("request_reputation_airdrop_transaction: drone_addr={} id={} reputations={} blockhash={}",
    //     drone_addr, id, reputations, blockhash).to_string())
    // );
    println!("{}",
        printLn(
            format!("request_reputation_airdrop_transaction: drone_addr={} id={} reputations={} blockhash={}",
                drone_addr, id, reputations, blockhash).to_string(),
            module_path!().to_string()
        )
    );
    // TODO: make this async tokio client
    let mut stream = TcpStream::connect_timeout(drone_addr, Duration::new(3, 0))?;
    stream.set_read_timeout(Some(Duration::new(10, 0)))?;
    let req = DroneRequest::GetReputation {
        reputations,
        blockhash,
        to: *id,
    };
    let req = serialize(&req).expect("serialize drone request");
    stream.write_all(&req)?;

    // Read length of transaction
    let mut buffer = [0; 2];
    stream.read_exact(&mut buffer).or_else(|err| {
        // info!(
        //     "{}",
        //     Info(format!("request_reputation_airdrop_transaction: buffer length read_exact error: {:?}",
        //     err).to_string())
        // );
        println!("{}",
            printLn(
                format!("request_reputation_airdrop_transaction: buffer length read_exact error: {:?}",
                    err).to_string(),
                module_path!().to_string()
            )
        );
        Err(Error::new(ErrorKind::Other, "Airdrop failed"))
    })?;
    let transaction_length = LittleEndian::read_u16(&buffer) as usize;
    if transaction_length >= PACKET_DATA_SIZE {
        Err(Error::new(
            ErrorKind::Other,
            format!(
                "request_reputation_airdrop_transaction: invalid transaction_length from drone: {}",
                transaction_length
            ),
        ))?;
    }

    // Read the transaction
    let mut buffer = Vec::new();
    buffer.resize(transaction_length, 0);
    stream.read_exact(&mut buffer).or_else(|err| {
        // info!(
        //     "{}",
        //     Info(format!("request_reputation_airdrop_transaction: buffer read_exact error: {:?}",
        //     err).to_string())
        // );
        println!("{}",
            printLn(
                format!("request_reputation_airdrop_transaction: buffer read_exact error: {:?}",
                    err).to_string(),
                module_path!().to_string()
            )
        );
        Err(Error::new(ErrorKind::Other, "Airdrop failed"))
    })?;

    let transaction: Transaction = deserialize(&buffer).or_else(|err| {
        Err(Error::new(
            ErrorKind::Other,
            format!("request_reputation_airdrop_transaction deserialize failure: {:?}", err),
        ))
    })?;
    Ok(transaction)
}

// For integration tests. Listens on random open port and reports port to Sender.
pub fn run_local_drone(
    mint_keypair: Keypair,
    sender: Sender<SocketAddr>,
    request_cap_input: Option<u64>,
) {
    thread::spawn(move || {
        let drone_addr = socketaddr!(0, 0);
        let drone = Arc::new(Mutex::new(Drone::new(
            mint_keypair,
            None,
            request_cap_input,
        )));
        run_drone(drone, drone_addr, Some(sender));
    });
}

pub fn run_drone(
    drone: Arc<Mutex<Drone>>,
    drone_addr: SocketAddr,
    send_addr: Option<Sender<SocketAddr>>,
) {
    let socket = TcpListener::bind(&drone_addr).unwrap();
    if send_addr.is_some() {
        send_addr
            .unwrap()
            .send(socket.local_addr().unwrap())
            .unwrap();
    }
    // info!("{}", Info(format!("Drone started. Listening on: {}", drone_addr).to_string()));
    println!("{}",
        printLn(
            format!("Drone started. Listening on: {}", drone_addr).to_string(),
            module_path!().to_string()
        )
    );
    let done = socket
        .incoming()
        .map_err(|e| debug!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            let drone2 = drone.clone();
            let framed = BytesCodec::new().framed(socket);
            let (writer, reader) = framed.split();

            let processor = reader.and_then(move |bytes| {
                match drone2.lock().unwrap().process_drone_request(&bytes) {
                    Ok(response_bytes) => {
                        trace!("Airdrop response_bytes: {:?}", response_bytes.to_vec());
                        Ok(response_bytes)
                    }
                    Err(e) => {
                        // info!("{}", Info(format!("Error in request: {:?}", e).to_string()));
                        println!("{}",
                            printLn(
                                format!("Error in request: {:?}", e).to_string(),
                                module_path!().to_string()
                            )
                        );
                        Ok(Bytes::from(&b""[..]))
                    }
                }
            });
            let server = writer
                .send_all(processor.or_else(|err| {
                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Drone response: {:?}", err),
                    ))
                }))
                .then(|_| Ok(()));
            tokio::spawn(server)
        });
    tokio::run(done);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BufMut;
    use morgan_interface::system_instruction::SystemInstruction;
    use std::time::Duration;

    #[test]
    fn test_check_request_limit() {
        let keypair = Keypair::new();
        let mut drone = Drone::new(keypair, None, Some(3));
        assert!(drone.check_request_limit(1));
        drone.request_current = 3;
        assert!(!drone.check_request_limit(1));
    }

    #[test]
    fn test_clear_request_count() {
        let keypair = Keypair::new();
        let mut drone = Drone::new(keypair, None, None);
        drone.request_current = drone.request_current + 256;
        assert_eq!(drone.request_current, 256);
        drone.clear_request_count();
        assert_eq!(drone.request_current, 0);
    }

    #[test]
    fn test_add_ip_to_cache() {
        let keypair = Keypair::new();
        let mut drone = Drone::new(keypair, None, None);
        let ip = "127.0.0.1".parse().expect("create IpAddr from string");
        assert_eq!(drone.ip_cache.len(), 0);
        drone.add_ip_to_cache(ip);
        assert_eq!(drone.ip_cache.len(), 1);
        assert!(drone.ip_cache.contains(&ip));
    }

    #[test]
    fn test_clear_ip_cache() {
        let keypair = Keypair::new();
        let mut drone = Drone::new(keypair, None, None);
        let ip = "127.0.0.1".parse().expect("create IpAddr from string");
        assert_eq!(drone.ip_cache.len(), 0);
        drone.add_ip_to_cache(ip);
        assert_eq!(drone.ip_cache.len(), 1);
        drone.clear_ip_cache();
        assert_eq!(drone.ip_cache.len(), 0);
        assert!(drone.ip_cache.is_empty());
    }

    #[test]
    fn test_drone_default_init() {
        let keypair = Keypair::new();
        let time_slice: Option<u64> = None;
        let request_cap: Option<u64> = None;
        let drone = Drone::new(keypair, time_slice, request_cap);
        assert_eq!(drone.time_slice, Duration::new(TIME_SLICE, 0));
        assert_eq!(drone.request_cap, REQUEST_CAP);
    }

    #[test]
    fn test_drone_build_airdrop_transaction() {
        let to = Pubkey::new_rand();
        let blockhash = Hash::default();
        let request = DroneRequest::GetAirdrop {
            difs: 2,
            to,
            blockhash,
        };

        let mint = Keypair::new();
        let mint_pubkey = mint.pubkey();
        let mut drone = Drone::new(mint, None, None);

        let tx = drone.build_airdrop_transaction(request).unwrap();
        let message = tx.message();

        assert_eq!(tx.signatures.len(), 1);
        assert_eq!(
            message.account_keys,
            vec![mint_pubkey, to, Pubkey::default()]
        );
        assert_eq!(message.recent_blockhash, blockhash);

        assert_eq!(message.instructions.len(), 1);
        let instruction: SystemInstruction = deserialize(&message.instructions[0].data).unwrap();
        assert_eq!(
            instruction,
            SystemInstruction::CreateAccount {
                difs: 2,
                reputations: 0,
                space: 0,
                program_id: Pubkey::default()
            }
        );

        let mint = Keypair::new();
        drone = Drone::new(mint, None, Some(1));
        let tx = drone.build_airdrop_transaction(request);
        assert!(tx.is_err());
    }

    #[test]
    fn test_drone_build_reputation_airdrop_transaction() {
        let to = Pubkey::new_rand();
        let blockhash = Hash::default();
        let request = DroneRequest::GetReputation {
            reputations: 2,
            to,
            blockhash,
        };

        let mint = Keypair::new();
        let mint_pubkey = mint.pubkey();
        let mut drone = Drone::new(mint, None, None);

        let tx = drone.build_airdrop_transaction(request).unwrap();
        let message = tx.message();

        assert_eq!(tx.signatures.len(), 1);
        assert_eq!(
            message.account_keys,
            vec![mint_pubkey, to, Pubkey::default()]
        );
        assert_eq!(message.recent_blockhash, blockhash);

        assert_eq!(message.instructions.len(), 1);
        let instruction: SystemInstruction = deserialize(&message.instructions[0].data).unwrap();
        assert_eq!(
            instruction,
            SystemInstruction::CreateAccountWithReputation {
                reputations: 2,
                space: 0,
                program_id: Pubkey::default()
            }
        );

        let mint = Keypair::new();
        drone = Drone::new(mint, None, Some(1));
        let tx = drone.build_airdrop_transaction(request);
        assert!(tx.is_err());
    }

    #[test]
    fn test_process_drone_request() {
        let to = Pubkey::new_rand();
        let blockhash = Hash::new(&to.as_ref());
        let difs = 50;
        let req = DroneRequest::GetAirdrop {
            difs,
            blockhash,
            to,
        };
        let req = serialize(&req).unwrap();
        let mut bytes = BytesMut::with_capacity(req.len());
        bytes.put(&req[..]);

        let keypair = Keypair::new();
        let expected_instruction =
            system_instruction::create_user_account(&keypair.pubkey(), &to, difs);
        let message = Message::new(vec![expected_instruction]);
        let expected_tx = Transaction::new(&[&keypair], message, blockhash);
        let expected_bytes = serialize(&expected_tx).unwrap();
        let mut expected_vec_with_length = vec![0; 2];
        LittleEndian::write_u16(&mut expected_vec_with_length, expected_bytes.len() as u16);
        expected_vec_with_length.extend_from_slice(&expected_bytes);

        let mut drone = Drone::new(keypair, None, None);
        let response = drone.process_drone_request(&bytes);
        let response_vec = response.unwrap().to_vec();
        assert_eq!(expected_vec_with_length, response_vec);

        let mut bad_bytes = BytesMut::with_capacity(9);
        bad_bytes.put("bad bytes");
        assert!(drone.process_drone_request(&bad_bytes).is_err());
    }

    #[test]
    fn test_process_reputation_drone_request() {
        let to = Pubkey::new_rand();
        let blockhash = Hash::new(&to.as_ref());
        let reputations = 50;
        let req = DroneRequest::GetReputation {
            reputations,
            blockhash,
            to,
        };
        let req = serialize(&req).unwrap();
        let mut bytes = BytesMut::with_capacity(req.len());
        bytes.put(&req[..]);

        let keypair = Keypair::new();
        let expected_instruction =
            system_instruction::create_user_account_with_reputation(&keypair.pubkey(), &to, reputations);
        let message = Message::new(vec![expected_instruction]);
        let expected_tx = Transaction::new(&[&keypair], message, blockhash);
        let expected_bytes = serialize(&expected_tx).unwrap();
        let mut expected_vec_with_length = vec![0; 2];
        LittleEndian::write_u16(&mut expected_vec_with_length, expected_bytes.len() as u16);
        expected_vec_with_length.extend_from_slice(&expected_bytes);

        let mut drone = Drone::new(keypair, None, None);
        let response = drone.process_drone_request(&bytes);
        let response_vec = response.unwrap().to_vec();
        assert_eq!(expected_vec_with_length, response_vec);

        let mut bad_bytes = BytesMut::with_capacity(9);
        bad_bytes.put("bad bytes");
        assert!(drone.process_drone_request(&bad_bytes).is_err());
    }
}
