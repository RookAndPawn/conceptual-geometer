use std::{convert::TryInto, path::PathBuf, str::FromStr};

use conceptual_geometer_core::internal::{PluginDeclaration, model::{CgVersion, RustcVersion}};
use libloading::Library;
use regex::Regex;
use tokio::fs::DirEntry;

use super::plugin_descriptor::PluginDescriptor;

lazy_static::lazy_static! {
    static ref FILE_NAME_REGEX: Regex = Regex::new(
        r"(?i).*\.dylib"
    ).unwrap();
}

/// Plugin declaration that can be used after plugin is unloaded
#[derive(Debug, Clone)]
struct PluginDeclarationCopy {
    pub rustc_version: String,
    pub core_version: String,
}

impl From<PluginDeclaration> for PluginDeclarationCopy {
    fn from(source: PluginDeclaration) -> Self {
        PluginDeclarationCopy {
            rustc_version: source.rustc_version.into(),
            core_version: source.core_version.into()
        }
    }
}

pub struct PluginDylib {
    path: PathBuf,
    rustc_version: RustcVersion,
    cg_version: CgVersion,
    plugin_version: String,
    name: String
}

impl PluginDylib {

    fn load_descriptor(path: &PathBuf) -> Option<PluginDeclarationCopy> {
         // load the library into memory
         unsafe {
            let library = Library::new(path).ok()?;

            // get a pointer to the plugin_declaration symbol.
            let decl = library
                .get::<*mut PluginDeclaration>(b"plugin_declaration\0").ok()?
                .read();

            let result = decl.into();

            library.close().ok()?;

            Some(result)
         }
    }

    pub async fn try_new(entry: &DirEntry) -> Option<Self> {
        let path = entry.path();

        let PluginDeclarationCopy {
            rustc_version ,
            core_version
        } = PluginDylib::load_descriptor(&path)?;

        Some(PluginDylib {
            path,
            rustc_version: RustcVersion::from_str(&rustc_version).ok()?,
            cg_version: CgVersion::from_str(&core_version).ok()?,
        })

    }

}