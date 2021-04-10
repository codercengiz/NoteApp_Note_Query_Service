
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

mod settings;
mod services;
mod models;

use crate::settings::Settings;
use std::env;


#[tokio::main]
#[warn(unused_must_use)]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let settings = Settings::init();
    
    services::run(settings).await;
}