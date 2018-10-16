use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;
use std::str::FromStr;

use super::{Address, Hash256};

#[derive(Debug, Clone)]
pub struct Config {
    pub path : PathBuf,

    pub host: String,
    pub port: String,

    pub sender : Address,
    pub private_key : Hash256,
}

impl Config {
    pub fn new(f_config: String) -> Self {
        let p = PathBuf::from(f_config.to_string());
        Config {
            path: p,
            host: "127.0.0.1".to_string(),
            port: "4000".to_string(),

            sender: Address::default(),
            private_key: Hash256::default(),
        }
    }

    pub fn init_params(&mut self) {
        let mut fd = match File::open(self.path.as_path()) {
            Ok(fd) => fd,
            Err(..) => {
                panic!("[YamlFileHandler Error]: Cannot open the following destination");
            }
        };
        let mut contents = String::new();
        let _ = fd.read_to_string(&mut contents);
        println!("content {:?}", contents);
        let content_vec: Vec<Yaml> = match YamlLoader::load_from_str(&contents) {
            Ok(vec) => vec,
            Err(e) => {
                panic!("[YamlLoader Error]: {}", e);
            },
        };

        let config_contents = &content_vec[0];

        println!("config contents {:?}", config_contents);

        self.host = config_contents["host"].clone().into_string().expect("get host error");
        self.port = config_contents["port"].clone().into_i64().expect("get port error").to_string();
        self.sender = Address::from_str(&config_contents["sender"].clone().into_string().expect("get sender error")).expect("address trans error");
        self.private_key = Hash256::from_str(&config_contents["private_key"].clone().into_string().expect("get private key error")).expect("private key trans error");
    }
}
