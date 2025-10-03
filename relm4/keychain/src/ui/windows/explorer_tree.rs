use std::cell::Ref;

use gtk::{
    prelude::*,
    gio,
    glib
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



#[derive(Debug, Clone)]
pub struct TreeNode {
    name: String,
    children: Vec<TreeNode>
}


impl TreeNode {
    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn children(&self) -> Vec<TreeNode> {
        return self.children.clone();
    }
}



pub struct ExplorerTree {

}

pub struct ExplorerTreeWidget {
    tv: gtk::ColumnView
}


// #[relm4::component(pub)]
impl SimpleComponent for ExplorerTree {
    type Input = ExplorerMessage;
    type Output = ApplicationMessage;
    type Init = ();
    type Widgets = ExplorerTreeWidget;
    type Root = gtk::Box;

    fn init_root() -> Self::Root {
        return gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .hexpand(true)
            .build();
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>
    ) -> ComponentParts<Self> {
        let sample = vec![
            TreeNode {
                name: "node_1".into(),
                children: vec![
                    TreeNode {
                        name: "node_1_1".into(),
                        children: vec![]
                    },
                    TreeNode {
                        name: "node_1_2".into(),
                        children: vec![]
                    }
                ]
            },
            TreeNode {
                name: "node_2".into(),
                children: vec![
                    TreeNode {
                        name: "node_2_1".into(),
                        children: vec![]
                    },
                    TreeNode {
                        name: "node_2_2".into(),
                        children: vec![]
                    }
                ]
            },
        ];

        let store = gio::ListStore::new::<glib::BoxedAnyObject>();

        for node in sample {
            store.append(&glib::BoxedAnyObject::new(node));    
        }

        let model = gtk::TreeListModel::new(
            store,
            false,
            false,
            |item| {
                let node = item.downcast_ref::<glib::BoxedAnyObject>().unwrap();
                let node: Ref<TreeNode> = node.borrow();

                let child_store = gio::ListStore::new::<glib::BoxedAnyObject>();
                for child in &node.children() {
                    child_store.append(&glib::BoxedAnyObject::new(child.clone()));
                }
                return Some(child_store.upcast());
            }
        );
        let selection = gtk::SingleSelection::builder()
            .model(&model)
            .build();

        let tv = gtk::ColumnView::builder()
            .model(&selection)
            .build();


        let name_factory = gtk::SignalListItemFactory::new();
        name_factory.connect_setup(|_, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let expander = gtk::TreeExpander::new();
            let label = gtk::Label::new(None);
            label.set_xalign(0.0);
            expander.set_child(Some(&label));
            item.set_child(Some(&expander));
        });

        name_factory.connect_bind(|_, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let row: gtk::TreeListRow = item.property("item");

            if let Some(inner) = row.item() {
                let node: Ref<TreeNode> = inner.downcast_ref::<glib::BoxedAnyObject>().unwrap().borrow();
                let name = node.name();
                if let Some(expander) = item.child().and_downcast::<gtk::TreeExpander>() {
                    if let Some(child) = expander.child().and_downcast::<gtk::Label>() {
                        child.set_label(&name);
                    }
                    expander.set_list_row(Some(&row));
                }
            }
        });

        let name_column = gtk::ColumnViewColumn::builder()
            .title("Name")
            .factory(&name_factory)
            .build();

        tv.append_column(&name_column);

        let sw = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .build();

        sw.set_child(Some(&tv));
        root.append(&sw);


        let model = ExplorerTree {};
        let widgets = ExplorerTreeWidget {
            tv
        };

        return ComponentParts{
            model,
            widgets
        };
    }
}