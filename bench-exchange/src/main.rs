pub mod bench;
mod cli;
pub mod order_book;

#[cfg(test)]
#[macro_use]
extern crate morgan_exchange_controller;

use crate::bench::{airdrop_difs, do_bench_exchange, Config};
use log::*;
use morgan::gossipService::{discover_cluster, get_clients};
use morgan_interface::signature::KeypairUtil;
use ansi_term::Color::{Green};
use morgan_helper::logHelper::*;

fn main() {
    morgan_logger::setup();
    morgan_metricbot::set_panic_hook("bench-exchange");

    let matches = cli::build_args().get_matches();
    let cli_config = cli::extract_args(&matches);

    let cli::Config {
        entrypoint_addr,
        drone_addr,
        identity,
        threads,
        num_nodes,
        duration,
        transfer_delay,
        fund_amount,
        batch_size,
        chunk_size,
        account_groups,
        ..
    } = cli_config;

    // info!("{}",
    //     Info(format!("Connecting to the cluster").to_string()));
    let info:String = format!("Connecting to the cluster").to_string();
    println!("{}",
        printLn(
            info,
            module_path!().to_string()
        )
    );

    let (nodes, _replicators) =
        discover_cluster(&entrypoint_addr, num_nodes).unwrap_or_else(|_| {
            panic!("Failed to discover nodes");
        });

    let clients = get_clients(&nodes);

    // info!("{}",
    //         Info(format!("{} nodes found", clients.len()).to_string()));
    let info:String = format!("{} nodes found", clients.len()).to_string();
    println!("{}",
        printLn(
            info,
            module_path!().to_string()
        )
    );

    if clients.len() < num_nodes {
        panic!("Error: Insufficient nodes discovered");
    }

    // info!("{}",
    //         Info(format!("Funding keypair: {}", identity.pubkey()).to_string()));
    let info:String = format!("Funding keypair: {}", identity.pubkey()).to_string();
    println!("{}",
        printLn(
            info,
            module_path!().to_string()
        )
    );
    let accounts_in_groups = batch_size * account_groups;
    const NUM_SIGNERS: u64 = 2;
    airdrop_difs(
        &clients[0],
        &drone_addr,
        &identity,
        fund_amount * (accounts_in_groups + 1) as u64 * NUM_SIGNERS,
    );

    let config = Config {
        identity,
        threads,
        duration,
        transfer_delay,
        fund_amount,
        batch_size,
        chunk_size,
        account_groups,
    };

    do_bench_exchange(clients, config);
}
