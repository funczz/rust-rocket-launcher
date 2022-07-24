use std::rc::Rc;

use gtk::{
    prelude::{BoxExt, GtkWindowExt},
    Orientation,
};

use crate::ui;

pub struct LaunchedWindow {
    pub count: Option<u8>,
}

impl ui::Window for LaunchedWindow {
    fn build_window(&self, window: Rc<gtk::ApplicationWindow>) {
        let window_ = Rc::clone(&window);

        let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
        window_.set_child(Some(&gtk_box));

        let label = Rc::new(gtk::Label::new(Some(&"launched.")));
        gtk_box.append(&*label);

        window_.present();
    }
}
