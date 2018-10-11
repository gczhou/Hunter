extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate blockchain_types as types;
extern crate ethcore_bigint as bigint;
extern crate rlp;
#[macro_use]
extern crate clap;
use clap::App;

mod case;
mod core;

mod config;

use self::core::Commands;
use self::config::Config;

fn main() {
    // cli.yaml文件需要放置在当前目录下
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    //println!("matches {:?}", matches);

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config_file = matches.value_of("config").unwrap_or("./config.yaml");
    println!("Value for config: {}", config_file);
    let mut config = Config::new(config_file.to_string());
    println!("Value for config: {:?}", config);

    config.init_params();
    println!("Value for config: {:?}", config);

    match matches.subcommand() {
        ("send_transaction", Some(sub_match)) => {
            let tx = sub_match.value_of("transaction").unwrap_or("");
            println!("send_tx {:?}", tx);
            Commands::send_transaction(tx.to_string());
        },
        ("send_raw_tx", Some(sub_match)) => {
            let raw_tx = sub_match.value_of("transaction").unwrap();
            println!("send raw transaction {:?}", raw_tx);
        },
        ("get_receipt_by_hash", Some(sub_match)) => {
            let hash = sub_match.value_of("hash").unwrap();
            println!("get_receipt_by_hash {:?}", hash);
        },
        ("get_block_by_number", Some(sub_match)) => {
            let number = sub_match.value_of("number").unwrap();
            println!("get_block_by_number {:?}", number);
        },
        ("get_blocknumber", None) => {
            println!("get_blocknumber");
        },
        ("get_chainstate", None) => {
            println!("get_chainstate");
        },
        _ => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
    }
}
