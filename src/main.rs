mod argument;
mod config;

use argument::Argument;
use clap::Parser;
use config::Config;
use std::{fs::File, io::BufReader, thread::sleep, time::Duration};
use twistercore_rpc::{Auth, Client, RpcApi, jsonrpc::serde_json};

fn main() {
    let argument = Argument::parse();
    let config: Vec<Config> =
        serde_json::from_reader(BufReader::new(File::open(argument.config).unwrap())).unwrap();

    if config.is_empty() {
        panic!("[{}] at least one ad is required to continue!", now())
    }
    for (n, ad) in config.iter().enumerate() {
        if ad.message.is_empty() {
            panic!("[{}] message for ad #{n} should not be empty!", now())
        }
        if ad.message.len() > 140 {
            panic!(
                "[{}] message length in ad #{n} reached 140 bytes limit!",
                now()
            )
        }
    }

    let len = config.len();
    let mut block: u64 = 0;
    let mut index: usize = 0;
    let mut total_rotations: usize = 0;
    let mut is_exit_request: bool = false;

    loop {
        match Client::new(
            &format!("{}://{}:{}", argument.scheme, argument.host, argument.port),
            Auth::UserPass(argument.user.to_string(), argument.password.to_string()),
        ) {
            Ok(rpc) => {
                println!(
                    "[{}] begin new connection to {}:{}..",
                    now(),
                    argument.host,
                    argument.port
                );
                loop {
                    match rpc.get_block_count() {
                        Ok(height) => match rpc.set_generate(true, argument.jobs) {
                            Ok(()) => {
                                if height > block {
                                    println!("[{}] block #{height}", now());
                                    if is_exit_request {
                                        match rpc.set_generate(false, argument.jobs) {
                                            Ok(()) => {
                                                println!(
                                                    "[{}] miner disabled as end of queue, exit.",
                                                    now()
                                                );
                                                return;
                                            }
                                            Err(e) => {
                                                panic!("[{}] could not stop the miner: {e}", now())
                                            }
                                        }
                                    } else if index == 0 {
                                        println!("[{}] begin new rotation..", now());
                                    }
                                    for ad in &config {
                                        if !user_exists(
                                            &ad.username,
                                            rpc.list_wallet_users().unwrap(),
                                        ) {
                                            panic!(
                                                "[{}] username @{} does not exist for this connection!",
                                                now(),
                                                ad.username
                                            )
                                        }
                                    }
                                    let number = index + 1;
                                    match rpc.set_spam_message(
                                        &config[index].username,
                                        &config[index].message,
                                        Some("replace"),
                                    ) {
                                        Ok(m) => println!(
                                            "[{}] ad changed to #{number} by @{} {:?}",
                                            now(),
                                            &config[index].username,
                                            m
                                        ),
                                        Err(e) => {
                                            eprintln!("[{}] could not update ad: {e}", now());
                                            break;
                                        }
                                    }
                                    if len > number {
                                        index += 1
                                    } else {
                                        index = 0;
                                        total_rotations += 1;
                                        if argument.mode == "s"
                                            && argument
                                                .rotations
                                                .is_some_and(|r| total_rotations > r)
                                        {
                                            is_exit_request = true
                                        }
                                    }
                                    block = height
                                } else {
                                    println!(
                                        "[{}] blockchain is up to date ({block}/{height})",
                                        now()
                                    )
                                }
                            }
                            Err(e) => {
                                eprintln!("[{}] could not start the miner: {e}", now());
                                break;
                            }
                        },
                        Err(e) => {
                            println!("[{}] could not get block count: {e}", now());
                            break;
                        }
                    }
                    println!(
                        "[{}] await {} seconds for new block to rotate..",
                        now(),
                        argument.delay
                    );
                    sleep(Duration::from_secs(argument.delay))
                }
            }
            Err(e) => println!("[{}] connection lost: {e}", now()),
        }
        println!("[{}] await {} seconds to reconnect..", now(), argument.wait);
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

fn now() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
