use gtk::{
    prelude::*,
};

use relm4::{
    prelude::*,
    // actions::*,
    ComponentParts,
    ComponentSender,
    SimpleComponent,
    typed_view::column::{
        LabelColumn,
        TypedColumnView
    }
};


use crate::messages::{
    application::ApplicationMessage,
    explorer::ExplorerMessage
};




#[derive(Debug, PartialEq, Eq)]
struct TreeItem {
    name: String
}

impl TreeItem {
    fn new(name: &str) -> Self {
        return Self {
            name: String::from(name)
        };
    }
}


struct NameColumn {

}


impl LabelColumn for NameColumn {
    type Item = TreeItem;
    type Value = String;

    const COLUMN_NAME: &'static str = "Name";
    const ENABLE_SORT: bool = false;
    const ENABLE_RESIZE: bool = false;

    fn get_cell_value(item: &Self::Item) -> Self::Value {
        return item.name.clone();
    }

    fn format_cell_value(value: &Self::Value) -> String {
        return value.to_string();
    }
}






pub struct Explorer {
    tv_wrapper: TypedColumnView<TreeItem, gtk::SingleSelection>
}


#[relm4::component(pub)]
impl SimpleComponent for Explorer {
    type Input = ExplorerMessage;
    type Output = ApplicationMessage;
    type Init = ();

    view!{
        gtk::Box {
            set_hexpand: true,

            gtk::ScrolledWindow {
                set_hexpand: true,
                set_vexpand: true,

                #[local_ref]
                tv -> gtk::ColumnView {

                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut tv_wrapper = TypedColumnView::<TreeItem, gtk::SingleSelection>::new();
        tv_wrapper.append_column::<NameColumn>();

        tv_wrapper.append(TreeItem::new("test 1"));
        tv_wrapper.append(TreeItem::new("test 2"));
        tv_wrapper.append(TreeItem::new("test 3"));

        let model = Explorer {
            tv_wrapper
        };

        let tv = &model.tv_wrapper.view;

        let widgets = view_output!();


        return ComponentParts {
            widgets,
            model
        };
    }
}
