extern crate env_logger;
pub mod path_utils;
pub mod tui;
pub mod ysnp;

use log::{debug, error};
use ysnp::YSNP;

fn main() {
    env_logger::init();

    let mut _ysnp = YSNP::new();
    let _ = _ysnp.add_dir("/hi").map_err(|e| {
        error!("could not add dir: {}", e);
    });
    debug!("ysnp sees dirs: {:#?}", _ysnp.dirs);
    let _tui = tui::Tui::new(&_ysnp);
    let _ = _tui.run_tui();
}
