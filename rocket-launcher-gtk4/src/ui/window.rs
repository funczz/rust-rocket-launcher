use std::rc::Rc;

use gtk::ApplicationWindow;

pub trait Window: Sized {
    #[allow(unused)]
    fn build_window(&self, window: Rc<ApplicationWindow>) {}
}
