
#[non_exhaustive]
#[derive(Debug, PartialEq, strum_macros::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum RustcVersion {
    #[cfg(not(release))]
    Dev,
    R1_54_0
}
