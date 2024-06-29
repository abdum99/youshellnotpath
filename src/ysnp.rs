use std::error::Error;
use std::fmt;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::str::FromStr;

use crate::path_utils;

type YSNPath = PathBuf;

pub struct YSNP {
    dirs: Vec<PathBuf>,
    raw_path: String, // to check if changed during run
}

impl YSNP {
    pub fn new() -> YSNP {
        let raw_path = path_utils::get_raw_path();
        YSNP::from_str(&raw_path).unwrap()
    }

    pub fn add_dir(&mut self, raw_dir: &str) -> Result<&Self, Box<dyn Error>> {
        let p = PathBuf::from(raw_dir);

        if !p.is_dir() {
            return Err("not a dir".into());
        }

        self.dirs.push(p);
        self.flush();
        Ok(self)
    }

    pub fn remove_dir(&mut self, to_remove: &PathBuf) -> Result<&Self, Box<dyn Error>> {
        let idx = self
            .dirs
            .iter()
            .position(|d| d == to_remove)
            .ok_or("dir not found in path")?;
        self.dirs.remove(idx);
        self.flush();
        Ok(self)
    }

    pub fn flush(&self) {
        path_utils::write_raw_path(&format!("{}", self))
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

impl fmt::Display for YSNP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("dirs: {:#?}", self.dirs);
        write!(
            f,
            "{}",
            self.dirs
                .iter()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<&str>>()
                .join(":")
        )
    }
}
