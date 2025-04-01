mod argument;
mod config;

use argument::Argument;
use clap::Parser;
use config::Config;
use std::{fs::File, io::BufReader, thread::sleep, time::Duration};
use twistercore_rpc::{Auth, Client, RpcApi, jsonrpc::serde_json};

fn main() {
    let argument = Argument::parse();
    let config: Config =
        serde_json::from_reader(BufReader::new(File::open(argument.config).unwrap())).unwrap();

    if config.rotate.is_empty() {
        panic!("At least one ad is required to continue!")
    }
    for (n, ad) in config.rotate.iter().enumerate() {
        if ad.message.is_empty() {
            panic!("Message for ad #{n} should not be empty!")
        }
        if ad.message.len() > 140 {
            panic!("Message length in ad #{n} reached 140 bytes limit!")
        }
    }

    let mut b: u64 = 0;
    let mut i: usize = 0;
    let l = config.rotate.len();

    loop {
        match Client::new(
            &format!("http://{}:{}", config.rpc.host, config.rpc.port),
            Auth::UserPass(config.rpc.user.to_string(), config.rpc.password.to_string()),
        ) {
            Ok(rpc) => {
                println!(
                    "Connection to {}:{} established!",
                    config.rpc.host, config.rpc.port
                );
                loop {
                    match rpc.get_block_count() {
                        Ok(block_count) => {
                            if block_count > b {
                                println!("Block #{block_count}");
                                match rpc.set_spam_message(
                                    &config.rotate[i].username,
                                    &config.rotate[i].message,
                                    Some("replace"),
                                ) {
                                    Ok(m) => println!(
                                        "Ad changed to #{i} by @{} {:?}",
                                        &config.rotate[i].username, m
                                    ),
                                    Err(e) => {
                                        println!("Could not update ad: {e}");
                                        break;
                                    }
                                }
                                if l > i + 1 {
                                    i += 1
                                } else {
                                    i = 0
                                }
                                b = block_count
                            } else {
                                println!("Blockchain is up to date ({b}/{block_count})")
                            }
                        }
                        Err(e) => {
                            println!("Could not get block count: {e}");
                            break;
                        }
                    }
                    println!(
                        "Await {} seconds for new block to rotate..",
                        argument.rotate
                    );
                    sleep(Duration::from_secs(argument.rotate))
                }
            }
            Err(e) => println!("Could not connect to client: {e}"),
        }
        println!("Await {} seconds to reconnect..", argument.rotate);
        sleep(Duration::from_secs(argument.rotate))
    }
}
