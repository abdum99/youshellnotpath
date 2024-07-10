extern crate env_logger;
mod tui;
mod ysnp;
mod utils;

use log::{debug, error};
use utils::path_utils;
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
