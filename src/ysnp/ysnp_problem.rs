use crate::path_utils;

#[derive(Debug)]
pub enum YSNProblem {
    Duplicate,
    IsFile,
    DirNotExist,
    DirEmpty,
}

fn get_problems(path: &str) -> Vec<YSNProblem> {
    path_utils::get_raw_path();
    // Check if valid 
    return vec![];
}
