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

    let len = config.rotate.len();
    let mut block: u64 = 0;
    let mut index: usize = 0;
    let mut is_exit_request: bool = false;

    loop {
        match Client::new(
            &format!(
                "{}://{}:{}",
                config.rpc.server.scheme, config.rpc.server.host, config.rpc.server.port
            ),
            match config.rpc.auth {
                Some(ref auth) => Auth::UserPass(auth.user.to_string(), auth.password.to_string()),
                None => Auth::None,
            },
        ) {
            Ok(rpc) => {
                println!(
                    "Connection to {}:{} established!",
                    config.rpc.server.host, config.rpc.server.port
                );
                loop {
                    match rpc.set_generate(true, argument.processors) {
                        Ok(()) => match rpc.get_block_count() {
                            Ok(height) => {
                                if height > block {
                                    println!("Block #{height}");
                                    if is_exit_request {
                                        match rpc.set_generate(false, argument.processors) {
                                            Ok(()) => {
                                                println!("Miner disabled, exit.");
                                                return;
                                            }
                                            Err(e) => panic!("Could not stop the miner: {e}"),
                                        }
                                    } else if index == 0 {
                                        println!("Begin new rotation..");
                                    }
                                    for ad in &config.rotate {
                                        if !user_exists(
                                            &ad.username,
                                            rpc.list_wallet_users().unwrap(),
                                        ) {
                                            panic!(
                                                "Username @{} does not exist for this connection!",
                                                ad.username
                                            )
                                        }
                                    }
                                    match rpc.set_spam_message(
                                        &config.rotate[index].username,
                                        &config.rotate[index].message,
                                        Some("replace"),
                                    ) {
                                        Ok(m) => println!(
                                            "Ad changed to #{index} by @{} {:?}",
                                            &config.rotate[index].username, m
                                        ),
                                        Err(e) => {
                                            eprintln!("Could not update ad: {e}");
                                            break;
                                        }
                                    }
                                    if len > index + 1 {
                                        index += 1
                                    } else {
                                        index = 0;
                                        if !argument.rotate {
                                            is_exit_request = true
                                        }
                                    }
                                    block = height
                                } else {
                                    eprintln!("Blockchain is up to date ({block}/{height})")
                                }
                            }
                            Err(e) => {
                                eprintln!("Could not get block count: {e}");
                                break;
                            }
                        },
                        Err(e) => {
                            eprintln!("Could start the miner: {e}");
                            break;
                        }
                    }
                    println!("Await {} seconds for new block to rotate..", argument.delay);
                    sleep(Duration::from_secs(argument.delay))
                }
            }
            Err(e) => eprintln!("Could not connect to client: {e}"),
        }
        println!("Await {} seconds to reconnect..", argument.wait);
        sleep(Duration::from_secs(argument.wait))
    }
}

fn user_exists(name: &str, names: Vec<String>) -> bool {
    if name == "nobody" {
        return true;
    }
    for value in names {
        if name == value {
            return true;
        }
    }
    false
}
