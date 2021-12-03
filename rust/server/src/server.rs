use eyre::Result;
use serde::Serialize;
use tokio::fs::read_dir;

use crate::{Config, plugin::{PluginBinary, PluginDylib}};

#[derive(Debug)]
pub struct Server {
    config: Config
}

impl Server {

    pub fn new(config: Config) -> Self {
        Server { config }
    }

    async fn find_plugin_binaries(&self) -> Result<Vec<PluginBinary>> {
        let mut dir_contents = read_dir(&self.config.plugin_bin_dir).await?;

        let mut result = Vec::new();

        while let Some(entry) = dir_contents.next_entry().await? {
            PluginBinary::try_new(&entry).await.map(|b| result.push(b));
        }

        Ok(result)
    }

    async fn find_plugin_dylibs(&self) -> Result<Vec<PluginDylib>> {
        let mut dir_contents = read_dir(&self.config.plugin_dir).await?;

        let mut result = Vec::new();

        while let Some(entry) = dir_contents.next_entry().await? {
            PluginDylib::try_new(&entry).await.map(|b| result.push(b));
        }

        Ok(result)
    }

    pub async fn run(self) -> Result<()> {
        // find the plugin processes
        let plugin_binaries = self.find_plugin_binaries().await?;

        // find the actual plugins
        let plugin_libraries = self.find_plugin_dylibs().await?;

        Ok(())
    }

}

