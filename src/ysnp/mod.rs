pub mod ysnp_problem;
pub mod ysnp_path;

use crate::path_utils;

pub use ysnp_path::YSNPath;

use std::error::Error;
use std::fmt;
use std::io::ErrorKind;
use std::str::FromStr;
use log::debug;


pub struct YSNP {
    pub dirs: Vec<YSNPath>,
    raw_path: String, // to check if changed during run
}

impl Default for YSNP {
    fn default() -> Self {
        Self::new()
    }
}

impl YSNP {
    pub fn new() -> YSNP {
        let raw_path = path_utils::get_raw_path();
        debug!("Creating YSNP from raw_path: {raw_path}");
        YSNP::from_str(&raw_path).unwrap()
    }

    // TODO: A bunch of checks to disallow adding faulty dirs
    pub fn add_dir(&mut self, raw_dir: &str) -> Result<&Self, Box<dyn Error>> {
        let p = YSNPath::from(raw_dir);

        if !p.buf.is_dir() {
            return Err("not a dir".into());
        }

        self.dirs.push(p);
        self.flush();
        Ok(self)
    }

    pub fn remove_dir(&mut self, to_remove: &YSNPath) -> Result<&Self, Box<dyn Error>> {
        let idx = self
            .dirs
            .iter()
            .position(|d| d == to_remove)
            .ok_or("dir not found in path")?;
        self.dirs.remove(idx);
        self.flush();
        Ok(self)
    }

    pub fn get_problems(&mut self) -> Vec<&YSNPath> {
        return self
            .dirs
            .iter()
            .filter(|p| !p.problems.is_empty())
            .collect();
    }

    fn flush(&self) {
        path_utils::write_raw_path(&format!("{}", self))
    }
}

impl FromStr for YSNP {
    type Err = ErrorKind;
    fn from_str(raw_path: &str) -> Result<Self, ErrorKind> {
        Ok(YSNP {
            dirs: raw_path
                .split(':')
                .map(YSNPath::from)
                // TODO: .filter(|p| p.is_dir()) (must note these changes and alert user)
                .collect(),
            raw_path: raw_path.to_string(),
        })
    }
}

impl fmt::Display for YSNP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.dirs
                .iter()
                .map(|p| p.buf.to_str().unwrap())
                .collect::<Vec<&str>>()
                .join(":")
        )
    }
}
