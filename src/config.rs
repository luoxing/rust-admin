use figment::{
    providers::{Format, Yaml},
    Figment,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{ffi::OsString, path::PathBuf, vec};

// casbin rbac model
pub static CASBIN_MODEL: &str = "[request_definition]
r = sub, dom, obj, act

[policy_definition]
p = sub, dom, obj, act

[role_definition]
g = _, _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub, r.dom) && r.dom == p.dom && r.obj == p.obj && r.act == p.act";

// 只要是配置文件中的配置项，都可以通过这个结构体来获取，
// 只要读取一次值后保存到内存，一直可供使用
pub static CFG: Lazy<Config> = Lazy::new(self::Config::init);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub path: Option<OsString>,
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
    pub pool_size: u32,
    pub pool_timeout: u32,
    pub log: bool,
    pub log_level: String,
}



impl Config {
    pub fn init() -> Config {
        // default find config file path
        let path: Vec<&str> = vec!["./config/", "./resource/config/"];
        let mut config = Config::default();
        for p in path {
            let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(p);
            let config_file = config_path.join("application.yaml");
            if config_file.exists() {
                config = Figment::new()
                    .merge(Yaml::file(&config_file))
                    .extract()
                    .unwrap();

                let config_path_active = config_path
                    .join(format!("application-{}.yaml", config.profiles.active));

                if config_path_active.exists() {
                    config = Figment::new()
                    .merge(Yaml::file(&config_file))
                    .merge(Yaml::file(&config_path_active))
                    .extract()
                    .unwrap();
                }
                config.path = Some(config_path.into_os_string());
                break;
            }
        }
        config
    }
}
