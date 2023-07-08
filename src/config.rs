use hocon::HoconLoader;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use lazy_static::lazy_static;

// create the config and its contents
pub fn create_config() -> std::io::Result<()> {
    let mut file = File::create("stuff.conf")?;
    file.write(b"token: Basic TOKEN\ndomain: DOMAIN")?;
    Ok(())
}

// read config stuff ig
#[derive(Deserialize, Debug)]
pub struct Config {
    pub token: String,
    pub domain: String,
}

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

// load the config
fn get_config() -> Config {
    let configs: Config = HoconLoader::new()
        .load_file("./stuff.conf")
        .expect("Config load err")
        .resolve()
        .expect("Config deserialize err");

    configs
}