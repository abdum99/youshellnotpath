pub mod path_utils;
pub mod ysnp;

use ysnp::YSNP;

fn main() {
    let _ysnp = YSNP::new();
    println!("Path:\n{}", _ysnp)
}
