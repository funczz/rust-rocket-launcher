use std::rc::Rc;

use rocket_launcher_core::sam;
use rust_sam::{SamModelPresent, SamStateNextAction, SamStateRepresentation};

use super::gtk4_sam_state_representation::Gtk4RocketLauncherSamStateRepresentation;

#[derive(Debug)]
pub struct Gtk4SamExecutor;

impl rust_sam::SamExecutor for Gtk4SamExecutor {
    type ActionData = sam::RocketLauncherSamData;
    type Error = sam::RocketLauncherSamError;
    type RepresentationData = Rc<gtk::ApplicationWindow>;
    type Result = ();
    type SamModel = sam::RocketLauncherSamModel;

    fn sam_present() -> fn(
        sam::RocketLauncherSamData,
    ) -> Result<sam::RocketLauncherSamModel, sam::RocketLauncherSamError> {
        sam::RocketLauncherSamModel::present
    }

    fn sam_state_next_action() -> fn(
        sam::RocketLauncherSamModel,
    ) -> Result<
        sam::RocketLauncherSamModel,
        sam::RocketLauncherSamError,
    > {
        sam::RocketLauncherSamState::next_action
    }

    fn sam_state_representation() -> fn(
        sam::RocketLauncherSamModel,
        Rc<gtk::ApplicationWindow>,
    ) -> Result<(), sam::RocketLauncherSamError> {
        Gtk4RocketLauncherSamStateRepresentation::representation
    }
}
