use std::{fmt::Display, io::ErrorKind, path::PathBuf, str::FromStr};

use crate::path_utils;

pub struct YSNP {
    dirs: Vec<PathBuf>,
    raw_path: String, // to check if changed during run
}

impl YSNP {
    pub fn new() -> YSNP {
        let raw_path = path_utils::get_raw_path();
        YSNP::from_str(&raw_path).unwrap()
    }
}

impl FromStr for YSNP {
    type Err = ErrorKind;
    fn from_str(raw_path: &str) -> Result<Self, ErrorKind> {
        Ok(YSNP {
            dirs: raw_path
                .split(":")
                .map(|dir_s| PathBuf::from(dir_s))
                // TODO: .filter(|p| p.is_dir()) (must note these changes and alert user)
                .collect(),
            raw_path: raw_path.to_string(),
        })
    }
}

impl Display for YSNP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.dirs)
    }
}
