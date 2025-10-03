use gtk::{
    prelude::*,
};

use relm4::{
    prelude::*,
    actions::*,
    ComponentParts,
    ComponentSender,
    SimpleComponent
};


use crate::messages::application::ApplicationMessage;
use crate::actions::*;


pub struct MainWindow {

}


#[relm4::component(pub)]
impl SimpleComponent for MainWindow {
    type Input = ApplicationMessage;
    type Output = ();
    type Init = ();

    menu!{
        main_menu: {
            "Database" {
                "New" => database_new::DatabaseNewAction
            }
        }
    }

    view!{
        #[root]
        main_window = gtk::ApplicationWindow {
            set_title: Some("KeyChain"),
            set_default_size: (800, 600),
            
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::PopoverMenuBar::from_model(Some(&main_menu)),

                gtk::ActionBar {
                    pack_start = &gtk::Button {
                        set_label: "Open",
                        set_icon_name: "document-open",
                        set_action_name: Some("win.database-new")
                    },
                    pack_start = &gtk::Button {
                        set_label: "Save",
                        set_icon_name: "document-save",
                        set_action_name: Some("win.database-new")
                    }
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = MainWindow {};
        let widgets = view_output!();

        let group = model.register_actions(sender);
        group.register_for_widget(&widgets.main_window);

        return ComponentParts {
            widgets,
            model
        };
    }
}


impl MainWindow {
    pub fn register_actions(&self, sender: ComponentSender<MainWindow>) -> RelmActionGroup<WindowActionGroup> {
        let mut group = RelmActionGroup::<WindowActionGroup>::new();

        let db_new_action = database_new::database_new_action(sender);
        group.add_action(db_new_action);

        return group;
    }
}