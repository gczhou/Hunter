use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use super::bigint::hash::{H256, H160 as Address};

#[derive(Debug, Clone)]
pub struct Config {
    pub path : PathBuf,

    pub host: String,
    pub port: String,

    pub sender : Address,
    pub private_key : H256,
}

impl Config {
    pub fn new(f_config: String) -> Self {
        let p = PathBuf::from(f_config.to_string());
        Config {
            path: p,
            host: "127.0.0.1".to_string(),
            port: "4000".to_string(),

            sender: Address::default(),
            private_key: H256::default(),
        }
    }

    pub fn init_params(&mut self) {
        println!("path {:?}", self.path.as_path());
        let mut fd = File::open(self.path.as_path());
        println!("fd {:?}", fd);
        match fd {
            Ok(fd) => fd,
            Err(..) => {
                panic!("[YamlFileHandler Error]: Cannot open the following destination");
            }
        };
        let mut content = String::new();
        //let _ = fd.read_to_string(&mut content);
    }
}
