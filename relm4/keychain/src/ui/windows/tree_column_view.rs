
use gtk::{
    prelude:;*,
    gio,
    glib
}


struct TreeNode {
    children: Vec<TreeNode>
}


pub struct TreeColumnView {

}


impl TreeColumnView {

    pub fn new() -> Self {
        let store = gio::ListStore::new::<glib::BoxedAnyObject>();
        let model = gtk::TreeListModel::new(
            store,
            false,
            false,
            |item| {
                let node = item.downcast_ref::<BoxedAnyObject>().unwrap();
                let node: Ref<TreeNode> = node.borrow();

                let child_store = gio::ListStore::new::<glib::BoxedAnyObject>();
                for child in &node.children() {
                    child_store.append(&BoxedAnyObject::new(child.clone()));
                }
                return Some(child_store.upcast());
            }
        );
        let seletion = gtk::SingleSelection;
        
        return Self {

        };
    }
}