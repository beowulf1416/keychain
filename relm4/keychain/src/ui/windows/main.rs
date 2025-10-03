
use std::convert::identity;

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
use crate::ui::windows::{
    explorer::Explorer,
    item::Item
};


pub struct MainWindow {
    explorer: Controller<Explorer>,
    item: Controller<Item>
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
                set_hexpand: true,

                gtk::PopoverMenuBar::from_model(Some(&main_menu)),

                gtk::ActionBar {
                    set_hexpand: true,

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

                gtk::Paned {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_hexpand: true,
                    set_vexpand: true,

                    #[wrap(Some)]
                    set_start_child = &gtk::Box {
                        append = model.explorer.widget(),
                    },

                    #[wrap(Some)]
                    set_end_child = &gtk::Box {
                        append = model.item.widget(),
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let explorer = Explorer::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let item = Item::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let model = MainWindow {
            explorer,
            item
        };
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