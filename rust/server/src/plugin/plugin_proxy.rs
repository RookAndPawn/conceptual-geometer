use std::path::PathBuf;

use super::PluginBinary;


#[derive(Debug)]
pub struct PluginProxy {
    binary: PluginBinary,
    plugin_path: PathBuf,
}