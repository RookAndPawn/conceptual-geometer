pub use server::Server;
pub use config::Config;
use clap::{App, Arg};
use tokio::runtime::Builder;
use core::panic;
use std::path::{PathBuf};
use tokio::fs::{File, create_dir_all};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use eyre::{Result, eyre};

mod server;
mod config;
mod plugin;

const CONFIG: &'static str = "config";

pub fn main() {

    let matches = App::new("Conceptual Geometer Server")
        .version(conceptual_geometer_core::CORE_VERSION)
        .author("Kevin G. <kevin.guthrie@gmail.com>")
        .about("Host application for Conceptual Geometer Server")
        .arg(Arg::with_name(CONFIG)
            .short("c")
            .long("config")
            .default_value("cg.toml"))
        .get_matches();

    let config_file = matches.value_of(CONFIG).expect("Config file has a default");

    let config_file_path = PathBuf::from(config_file);
    let write_config_file = !config_file_path.as_path().exists();

    let runtime = Builder::new_multi_thread()
        .enable_time()
        .build()
        .expect("Failed to create runtime");

    let server_result : Result<Server> = runtime.block_on(async move {

        if write_config_file {
            let parent_dir = config_file_path.parent().ok_or_else(|| eyre!("Invalid config file {:?}", config_file_path))?;
            create_dir_all(parent_dir).await?;
            let mut file = File::create(&config_file_path).await?;
            file.write_all(config::DEFAULT_CONFIG_FILE.as_bytes()).await?;
        }

        let mut file = File::open(&config_file_path).await?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let config : Config = toml::from_str(&contents)?;

        Ok(Server::new(config))
    });

    let server = match server_result {
        Ok(server) => server,
        Err(report) => {
            panic!("Failed to create server - {}", report);
        }
    };

    if let Err(report) = runtime.block_on(server.run()) {
        panic!("Server exited unexpectedly - {}", report);
    }

}