use std::{cell::Cell, rc::Rc};

use gtk::{
    glib::clone,
    prelude::{BoxExt, ButtonExt, EditableExt, EntryExt, GtkWindowExt, WidgetExt},
    Button, Orientation,
};
use rocket_launcher_core::{domain, sam};
use rust_sam::{SamAction, SamExecutor};

use crate::{gtk4_sam, ui};

pub struct ReadyWindow;

impl ReadyWindow {
    pub fn on_click_start_button(
        button: &Button,
        window: Rc<gtk::ApplicationWindow>,
        count: Option<u8>,
    ) {
        button.set_visible(false);
        let data = sam::RocketLauncherSamData::new(domain::RocketLauncherState::Ready, count);
        let _ = gtk4_sam::Gtk4SamExecutor::execute(
            sam::action::StartAction::execute,
            data.clone(),
            window,
        );
    }
}

impl ui::Window for ReadyWindow {
    fn build_window(&self, window: Rc<gtk::ApplicationWindow>) {
        let count: Rc<Cell<Option<u8>>> = Rc::new(Cell::new(None));

        let window_ = Rc::clone(&window);
        let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
        window_.set_child(Some(&gtk_box));

        let label = Rc::new(gtk::Label::new(Some("ready.")));

        let start_button = Rc::new(
            gtk::Button::builder()
                .label("start")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build(),
        );
        start_button.set_visible(false);

        start_button.connect_clicked(clone!(@strong window_, @strong count =>
            move |button| {
                Self::on_click_start_button(button, Rc::clone(&window_), count.get());
             }
        ));

        let input = gtk::Entry::new();
        input.set_placeholder_text(Some("10"));
        input.set_input_purpose(gtk::InputPurpose::Email);
        input.set_activates_default(true);
        input.connect_changed(clone!(@strong count, @strong start_button => move |e| {
            let text = e.text();
            if text == "" {
                start_button.set_visible(false);
                return
            }
            match text.parse() {
                Ok(v) => {
                    count.set(Some(v));
                    start_button.set_visible(true);
                },
                _ => {e.set_text("")}
            }
        }));

        gtk_box.append(&*label);
        gtk_box.append(&input);
        gtk_box.append(&*start_button);
        window_.present();
    }
}
