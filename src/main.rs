mod argument;
mod config;

use argument::Argument;
use clap::Parser;
use config::Config;
use std::{
    fs::File,
    io::BufReader,
    thread::sleep,
    time::{Duration, SystemTime},
};
use twistercore_rpc::{Auth, Client, RpcApi, jsonrpc::serde_json};

fn main() {
    let argument = Argument::parse();
    let config: Vec<Config> =
        serde_json::from_reader(BufReader::new(File::open(argument.config).unwrap())).unwrap();

    if config.is_empty() {
        panic!("[{}] at least one ad is required to continue!", now())
    }
    for (i, ad) in config.iter().enumerate() {
        let n = i + 1;
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

    let l = config.len();
    let mut best_block_time: u64 = 0;
    let mut ad_index: usize = 0;
    let mut iteration_total: usize = 0;
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
                    argument.port,
                );
                loop {
                    match rpc.get_best_block_hash() {
                        Ok(hash) => match rpc.get_block(&hash, true) {
                            Ok(block) => {
                                let block_time = block.time as u64;
                                if block_time > best_block_time {
                                    println!("[{}] block #{}", now(), block_time);
                                    if let Some(latency) = argument.latency {
                                        if latency
                                            > SystemTime::now()
                                                .duration_since(std::time::UNIX_EPOCH)
                                                .unwrap()
                                                .as_secs()
                                                - block_time
                                        {
                                            match rpc.set_generate(false, argument.jobs) {
                                                Ok(()) => {
                                                    println!(
                                                        "[{}] apply worker latency for {latency} seconds..",
                                                        now()
                                                    );
                                                    sleep(Duration::from_secs(latency))
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "[{}] could not stop the worker: {e}",
                                                        now()
                                                    );
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    best_block_time = block_time;
                                    if is_exit_request {
                                        match rpc.set_generate(false, argument.jobs) {
                                            Ok(()) => {
                                                println!(
                                                    "[{}] worker disabled as end of queue, exit.",
                                                    now()
                                                );
                                                return;
                                            }
                                            Err(e) => {
                                                panic!("[{}] could not stop the worker: {e}", now())
                                            }
                                        }
                                    } else if ad_index == 0 {
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
                                    let n = ad_index + 1;
                                    match rpc.set_spam_message(
                                        &config[ad_index].username,
                                        &config[ad_index].message,
                                        Some("replace"),
                                    ) {
                                        Ok(m) => println!(
                                            "[{}] ad changed to #{n} by @{} {:?}",
                                            now(),
                                            &config[ad_index].username,
                                            m
                                        ),
                                        Err(e) => {
                                            eprintln!("[{}] could not update ad: {e}", now());
                                            break;
                                        }
                                    }
                                    if l > n {
                                        ad_index += 1
                                    } else {
                                        ad_index = 0;
                                        iteration_total += 1;
                                        if argument.mode == "s"
                                            && argument
                                                .rotations
                                                .is_some_and(|r| iteration_total > r)
                                        {
                                            is_exit_request = true
                                        }
                                    }
                                    if let Err(e) = rpc.set_generate(true, argument.jobs) {
                                        eprintln!("[{}] could not start the worker: {e}", now());
                                        break;
                                    }
                                } else {
                                    println!(
                                        "[{}] blockchain is up to date (U{})",
                                        now(),
                                        block.time
                                    )
                                }
                            }
                            Err(e) => {
                                eprintln!("[{}] could not get block `{hash}`: {e}", now());
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
    for n in names {
        if name == n {
            return true;
        }
    }
    false
}

fn now() -> u128 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
