use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use gtk::{
    glib::{self, clone},
    prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt},
    Button, Orientation,
};
use rocket_launcher_core::{domain, sam, service};
use rust_sam::{SamAction, SamExecutor};

use crate::{gtk4_sam, ui};

pub struct CountingWindow {
    pub count: Option<u8>,
}

impl CountingWindow {
    pub fn new_sam_data(count: u8) -> sam::RocketLauncherSamData {
        sam::RocketLauncherSamData::new(domain::RocketLauncherState::Counting, Some(count))
    }

    pub fn format_count_label_text(count: Option<u8>) -> String {
        let c = match count {
            None => "".to_string(),
            Some(v) => v.to_string(),
        };
        format!("counting. {}", c)
    }

    pub fn execute_decrement_action(window: Rc<gtk::ApplicationWindow>, count: u8) {
        let _ = gtk4_sam::Gtk4SamExecutor::execute(
            sam::action::DecrementAction::execute,
            Self::new_sam_data(count),
            Rc::clone(&window),
        );
    }

    pub fn on_click_abort_button(button: &Button, window: Rc<gtk::ApplicationWindow>, count: u8) {
        button.set_visible(false);
        let _ = gtk4_sam::Gtk4SamExecutor::execute(
            sam::action::AbortAction::execute,
            Self::new_sam_data(count),
            window,
        );
    }
}

impl ui::Window for CountingWindow {
    fn build_window(&self, window: Rc<gtk::ApplicationWindow>) {
        match self.count {
            None => assert!(false),
            _ => {}
        }

        let count = Rc::new(self.count);
        let counting_timer = service::CountingTimer::new(count.unwrap());
        let counting_timer = Arc::new(Mutex::new(counting_timer));

        let text = Self::format_count_label_text(*count);
        let label = Rc::new(gtk::Label::new(Some(&*text)));

        let abort_button = Rc::new(
            gtk::Button::builder()
                .label("abort")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build(),
        );
        let abort_button_ = Rc::clone(&abort_button);
        abort_button_.connect_clicked({
            clone!(@strong window, @strong counting_timer =>
                move |_| {
                    counting_timer
                    .lock()
                    .unwrap()
                    .abort(|c| Self::on_click_abort_button(&abort_button, Rc::clone(&window), c));
                 }
            )
        });

        let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
        window.set_child(Some(&gtk_box));
        gtk_box.append(&*label);
        gtk_box.append(&*abort_button_);

        let (before_sender, before_receiver) =
            glib::MainContext::channel::<u8>(glib::PRIORITY_DEFAULT);
        let (after_sender, after_receiver) =
            glib::MainContext::channel::<u8>(glib::PRIORITY_DEFAULT);
        before_receiver.attach(None, {
            let window = Rc::clone(&window);
            move |c| {
                Self::execute_decrement_action(Rc::clone(&window), c);
                glib::Continue(true)
            }
        });
        after_receiver.attach(None, {
            move |c| {
                let text = Self::format_count_label_text(Some(c));
                label.set_text(&*text);
                glib::Continue(true)
            }
        });

        window.present();

        counting_timer.lock().unwrap().start(
            move |c| {
                before_sender.send(c).unwrap();
            },
            move |c| {
                after_sender.send(c).unwrap();
            },
        );
    }
}
