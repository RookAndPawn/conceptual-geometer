
#[non_exhaustive]
#[derive(Debug, PartialEq, strum_macros::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum CgVersion {
    #[cfg(not(release))]
    Dev,
    CG0_1_0
}