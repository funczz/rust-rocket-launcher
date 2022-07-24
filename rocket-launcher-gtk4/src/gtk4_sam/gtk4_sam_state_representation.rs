use std::rc::Rc;

use rocket_launcher_core::sam;
use rust_sam::SamStateRepresentation;

use crate::ui::{view, Window};

/**
 *  [Gtk4] SAM ステート ::representation 実装
 */

#[derive(Debug)]
pub struct Gtk4RocketLauncherSamStateRepresentation;

impl SamStateRepresentation for Gtk4RocketLauncherSamStateRepresentation {
    type Error = sam::RocketLauncherSamError;
    type RepresentationData = Rc<gtk::ApplicationWindow>;
    type Result = ();
    type SamModel = sam::RocketLauncherSamModel;

    fn representation(
        model: sam::RocketLauncherSamModel,
        representation_data: Rc<gtk::ApplicationWindow>,
    ) -> Result<(), sam::RocketLauncherSamError> {
        let window = Rc::clone(&representation_data);

        if sam::RocketLauncherSamState::is_ready(&model) == true {
            view::ReadyWindow.build_window(window)
        } else if sam::RocketLauncherSamState::is_transition_to_counting(&model) == true {
            view::CountingWindow {
                count: model.model.count,
            }
            .build_window(window)
        } else if sam::RocketLauncherSamState::is_launched(&model) == true {
            view::LaunchedWindow {
                count: model.model.count,
            }
            .build_window(window)
        } else if sam::RocketLauncherSamState::is_aborted(&model) == true {
            view::AbortedWindow {
                count: model.model.count,
            }
            .build_window(window)
        };
        Ok(())
    }
}
