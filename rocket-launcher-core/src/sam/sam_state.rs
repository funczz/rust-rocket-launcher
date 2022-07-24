use crate::domain;

use super::RocketLauncherSamModel;

/**
 * SAM ステート
 */

#[derive(Debug)]
pub struct RocketLauncherSamState;

impl RocketLauncherSamState {
    pub fn is_ready(model: &RocketLauncherSamModel) -> bool {
        match model.model.state {
            domain::RocketLauncherState::Ready => true,
            _ => false,
        }
    }

    pub fn is_transition_to_counting(model: &RocketLauncherSamModel) -> bool {
        let prev_model = match &model.prev_model {
            Some(v) => v,
            None => return false,
        };
        match (&model.model.state, &prev_model.state) {
            (domain::RocketLauncherState::Counting, domain::RocketLauncherState::Ready) => true,
            _ => false,
        }
    }

    pub fn is_counting(model: &RocketLauncherSamModel) -> bool {
        match model.model.state {
            domain::RocketLauncherState::Counting => true,
            _ => false,
        }
    }

    pub fn is_launched(model: &RocketLauncherSamModel) -> bool {
        match model.model.state {
            domain::RocketLauncherState::Launched => true,
            _ => false,
        }
    }

    pub fn is_aborted(model: &RocketLauncherSamModel) -> bool {
        match model.model.state {
            domain::RocketLauncherState::Aborted => true,
            _ => false,
        }
    }
}
