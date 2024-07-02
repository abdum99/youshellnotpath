pub mod path_utils;
pub mod tui;
pub mod ysnp;

use log::error;
use ysnp::YSNP;

fn main() {
    let mut _ysnp = YSNP::new();
    let _ = _ysnp.add_dir("/opt").map_err(|e| {
        error!("could not add dir: {}", e);
    });
    let _tui = tui::Tui::new(&_ysnp);
    _tui.run_tui();
}
