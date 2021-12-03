use std::path::PathBuf;
use tokio::fs::DirEntry;
use conceptual_geometer_core::internal::model::{CgVersion, RustcVersion};
use regex::Regex;
use std::str::FromStr;

lazy_static::lazy_static! {
    static ref FILE_NAME_REGEX: Regex = Regex::new(
        r"(?i)conceptual\-geometer\-plugin\-process(?:_r(\d+(?:\-\d+))_cg(\d+(?:\-\d+)))?"
    ).unwrap();
}

#[derive(Debug)]
pub struct PluginBinary {
    rustc_version: RustcVersion,
    cg_version: CgVersion,
    path: PathBuf
}

impl PluginBinary {

    pub async fn try_new(entry: &DirEntry) -> Option<Self> {
        if entry.file_type().await.ok()?.is_file() {
            return None
        }

        entry.path().file_name()?.to_str()
            .filter(|file_name| FILE_NAME_REGEX.is_match(file_name))
            .map(|file_name| {
                let captures = FILE_NAME_REGEX.captures(file_name)?;

                let rustc_version = captures.get(1)
                    .map(|m| m.as_str())
                    .or(Some("Dev"))
                    .map(RustcVersion::from_str)
                    .map(Result::ok)
                    .flatten()?;

                let cg_version = captures.get(2)
                    .map(|m| m.as_str())
                    .or(Some("Dev"))
                    .map(CgVersion::from_str)
                    .map(Result::ok)
                    .flatten()?;

                Some(PluginBinary {
                    path: entry.path(),
                    rustc_version,
                    cg_version
                })
            })
            .flatten()
    }

}