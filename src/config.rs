use figment::{
    providers::{Format, Yaml},
    Figment,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, vec};

//  只要是配置文件中的配置项，都可以通过这个结构体来获取，
// 只要读取一次值后保存到内存，一直可供使用
pub static CFG: Lazy<Config> = Lazy::new(self::Config::init);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub profiles: Profiles,
    pub app_name: String,
    pub server: Server,
    pub log: Log,
    pub db: DB,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Profiles {
    pub active: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Server {
    pub address: String,
    pub port: u16,
    pub tls: bool,
    pub pem_cert_path: String,
    pub pem_key_path: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Log {
    pub level: String,
    pub path: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DB {
    pub url: String,
    pub pool_size: u16,
    pub pool_timeout: u16,
}

impl Config {
    pub fn init() -> Config {
        // default find config file path
        let path: Vec<&str> = vec!["./", "./config/", "./resource/config/"];
        let mut config = Config::default();
        for p in path {
            let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(p)
                .join("application.yaml");
            if config_path.exists() {
                config = Figment::new()
                    .merge(Yaml::file(&config_path))
                    .extract()
                    .unwrap();

                let config_path_active = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join(&p)
                    .join(format!("application-{}.yaml", config.profiles.active));

                if config_path_active.exists() {
                    config = Figment::new()
                    .merge(Yaml::file(&config_path))
                    .merge(Yaml::file(&config_path_active))
                    .extract()
                    .unwrap();
                    break;
                }
            }
        }
        config
    }
}
