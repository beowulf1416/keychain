pub mod messages;
pub mod actions;
pub mod ui;

use relm4::{
    prelude::*,
};

use crate::ui::windows::main::MainWindow;


fn main() {
    env_logger::init();

    let app = RelmApp::new("org.devphilplus.keychain");
    app.run::<MainWindow>(());
}
