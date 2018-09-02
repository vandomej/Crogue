extern crate serde;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct Config {
    pub screen_width: i32,
    pub screen_height: i32,
    pub fps_limit: i32
}

lazy_static! {
    pub static ref CONFIG: Config = read_config();
}

fn read_config() -> Config {
    let mut file = File::open("config.toml").expect("unable to open");
    let mut text = String::new();
    file.read_to_string(&mut text);
    return toml::from_str(&text).unwrap();
}
