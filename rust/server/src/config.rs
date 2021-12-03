use std::path::PathBuf;

pub const DEFAULT_CONFIG_FILE : &'static str = include_str!("../resources/cg-default.toml");

#[derive(serde_derive::Deserialize, Debug)]
pub struct Config {
    pub plugin_dir: PathBuf,
    pub plugin_bin_dir: PathBuf,
    pub db_file: PathBuf
}