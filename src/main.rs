pub mod path_utils;
pub mod ysnp;
pub mod tui;

use log::error;
use ysnp::YSNP;

fn main() {
    let mut _ysnp = YSNP::new();
    let _ = _ysnp.add_dir("/opt").map_err(|e| {
        error!("could not add dir: {}", e);
    });
    tui::run_tui(&_ysnp);
    println!("Path:\n{}", _ysnp);
}
