use log::{
    debug
};

use relm4::{
    prelude::*,
    actions::*
};


use crate::actions::WindowActionGroup;
use crate::ui::windows::main::MainWindow;


relm4::new_stateless_action!(pub DatabaseNewAction, WindowActionGroup, "database-new");


pub fn database_new_action(_sender: ComponentSender<MainWindow>) -> RelmAction<DatabaseNewAction> {
    return RelmAction::new_stateless(move |_| {
        debug!("DatabaseNewAction action triggered");
    });
}