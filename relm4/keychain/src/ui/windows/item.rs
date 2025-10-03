use gtk::{
    prelude::*,
};

use relm4::{
    prelude::*,
    // actions::*,
    ComponentParts,
    ComponentSender,
    SimpleComponent
};


use crate::messages::{
    application::ApplicationMessage,
    explorer::ExplorerMessage
};
// use crate::actions::*;



pub struct Item {

}


#[relm4::component(pub)]
impl SimpleComponent for Item {
    type Input = ExplorerMessage;
    type Output = ApplicationMessage;
    type Init = ();

    view!{
        gtk::Box {
            set_hexpand: true,

            gtk::Notebook {
                set_hexpand: true,

                append_page[Some(
                    &gtk::Button::builder()
                        .icon_name("folder-visiting")
                        .tooltip_text("General")
                        .build()
                )] = &gtk::Box {
                    gtk::Label {
                        set_text: "General"
                    }
                },

                append_page[Some(
                    &gtk::Button::builder()
                        .icon_name("folder-visiting")
                        .tooltip_text("Advanced")
                        .build()
                )] = &gtk::Box {
                    gtk::Label {
                        set_text: "Advanced"
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Item {};
        let widgets = view_output!();


        return ComponentParts {
            widgets,
            model
        };
    }
}
