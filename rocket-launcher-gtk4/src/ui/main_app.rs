use std::rc::Rc;

use rocket_launcher_core::{domain, sam};
use rust_sam::SamExecutor;

use crate::gtk4_sam;

use super::App;

pub struct MainApp;

impl App for MainApp {
    fn build_window(window: Rc<gtk::ApplicationWindow>) {
        let model = sam::RocketLauncherSamModel::new(domain::RocketLauncherState::Ready, None);
        let _ = gtk4_sam::Gtk4SamExecutor::do_representation(model, window);
    }
}
