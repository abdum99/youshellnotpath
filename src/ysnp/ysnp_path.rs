use crate::ysnp::ysnp_problem::YSNProblem;

use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct YSNPath {
    pub buf: PathBuf,
    pub problems: Vec<YSNProblem>,
}

impl From<&str> for YSNPath {
    fn from(s: &str) -> YSNPath {
        YSNPath {
            buf: PathBuf::from(s),
            ..Default::default()
        }
    }
}

impl PartialEq for YSNPath {
    fn eq(&self, other: &Self) -> bool {
        self.buf == other.buf
    }
}

