use std::rc::Rc;

use gtk::{
    prelude::{ApplicationExt, ApplicationExtManual},
    Application, ApplicationWindow,
};

use crate::ui::global;

pub trait App: Sized + 'static {
    fn new_application(id: Option<&str>) -> Application {
        let id = match id {
            None => Self::application_id(),
            Some(v) => v,
        };
        Application::builder().application_id(id).build()
    }

    fn new_window(app: &Application, title: Option<&str>) -> ApplicationWindow {
        let title = match title {
            None => Self::window_title(),
            Some(v) => v,
        };
        ApplicationWindow::builder()
            .application(app)
            .title(title)
            .build()
    }

    fn application_id() -> &'static str {
        global::GTK4_APPLICATIN_ID
    }

    fn window_title() -> &'static str {
        global::GTK4_WINDOW_TITLE
    }

    fn start() -> Application {
        let app = Self::new_application(None);
        app.connect_activate(Self::build_app);
        app.run();
        app
    }

    fn build_app(app: &Application) {
        let window = Rc::new(Self::new_window(&app, None));
        Self::build_window(window);
    }

    #[allow(unused)]
    fn build_window(window: Rc<ApplicationWindow>) {}
}
