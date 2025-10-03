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



pub struct Explorer {

}


#[relm4::component(pub)]
impl SimpleComponent for Explorer {
    type Input = ExplorerMessage;
    type Output = ApplicationMessage;
    type Init = ();

    view!{
        gtk::Box {
            set_hexpand: true,

            gtk::Label {
                set_hexpand: true,
                set_label: "placeholder"

            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Explorer {};
        let widgets = view_output!();


        return ComponentParts {
            widgets,
            model
        };
    }
}
