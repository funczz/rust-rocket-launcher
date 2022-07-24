use std::rc::Rc;

use gtk::{
    prelude::{BoxExt, GtkWindowExt},
    Orientation,
};

use crate::ui;

pub struct AbortedWindow {
    pub count: Option<u8>,
}

impl ui::Window for AbortedWindow {
    fn build_window(&self, window: Rc<gtk::ApplicationWindow>) {
        let window_ = Rc::clone(&window);

        let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
        window_.set_child(Some(&gtk_box));

        let label = Rc::new(gtk::Label::new(Some(&"aborted.")));
        gtk_box.append(&*label);

        window_.present();
    }
}
