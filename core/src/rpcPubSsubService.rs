//! The `pubsub` module implements a threaded subscription service on client RPC request

use crate::rpcPubsub::{RpcSolPubSub, RpcSolPubSubImpl};
use crate::rpcSubscriptions::RpcSubscriptions;
use crate::service::Service;
use jsonrpc_pubsub::{PubSubHandler, Session};
use jsonrpc_ws_server::{RequestContext, ServerBuilder};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, sleep, Builder, JoinHandle};
use std::time::Duration;
use morgan_helper::logHelper::*;

pub struct PubSubService {
    thread_hdl: JoinHandle<()>,
}

impl Service for PubSubService {
    type JoinReturnType = ();

    fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }
}

impl PubSubService {
    pub fn new(
        subscriptions: &Arc<RpcSubscriptions>,
        pubsub_addr: SocketAddr,
        exit: &Arc<AtomicBool>,
    ) -> Self {
        // info!("{}", Info(format!("rpc_pubsub bound to {:?}", pubsub_addr).to_string()));
        println!("{}",
            printLn(
                format!("rpc_pubsub bound to {:?}", pubsub_addr).to_string(),
                module_path!().to_string()
            )
        );
        let rpc = RpcSolPubSubImpl::new(subscriptions.clone());
        let exit_ = exit.clone();
        let thread_hdl = Builder::new()
            .name("morgan-pubsub".to_string())
            .spawn(move || {
                let mut io = PubSubHandler::default();
                io.extend_with(rpc.to_delegate());

                let server = ServerBuilder::with_meta_extractor(io, |context: &RequestContext| {
                        // info!("{}", Info(format!("New pubsub connection").to_string()));
                        println!("{}",
                            printLn(
                                format!("New pubsub connection").to_string(),
                                module_path!().to_string()
                            )
                        );
                        let session = Arc::new(Session::new(context.sender().clone()));
                        session.on_drop(|| {
                            // info!("{}", Info(format!("Pubsub connection dropped").to_string()));
                            println!("{}",
                                printLn(
                                    format!("Pubsub connection dropped").to_string(),
                                    module_path!().to_string()
                                )
                            );
                        });
                        session
                })
                .start(&pubsub_addr);

                if let Err(e) = server {
                    // warn!("Pubsub service unavailable error: {:?}. \nAlso, check that port {} is not already in use by another application", e, pubsub_addr.port());
                    println!(
                        "{}",
                        Warn(
                            format!("Pubsub service unavailable error: {:?}. \nAlso, check that port {} is not already in use by another application", e, pubsub_addr.port()).to_string(),
                            module_path!().to_string()
                        )
                    );
                    return;
                }
                while !exit_.load(Ordering::Relaxed) {
                    sleep(Duration::from_millis(100));
                }
                server.unwrap().close();
            })
            .unwrap();
        Self { thread_hdl }
    }

    pub fn close(self) -> thread::Result<()> {
        self.join()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_pubsub_new() {
        let subscriptions = Arc::new(RpcSubscriptions::default());
        let pubsub_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
        let exit = Arc::new(AtomicBool::new(false));
        let pubsub_service = PubSubService::new(&subscriptions, pubsub_addr, &exit);
        let thread = pubsub_service.thread_hdl.thread();
        assert_eq!(thread.name().unwrap(), "morgan-pubsub");
    }
}
