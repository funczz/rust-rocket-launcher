use crate::domain;

use super::RocketLauncherSamData;

/**
 * SAM モデル
 */

#[derive(Debug, Clone)]
pub struct RocketLauncherSamModel {
    pub model: domain::RocketLauncherModel,
    pub prev_model: Option<domain::RocketLauncherModel>,
}

impl RocketLauncherSamModel {
    pub fn new(state: domain::RocketLauncherState, count: Option<u8>) -> Self {
        RocketLauncherSamModel {
            model: domain::RocketLauncherModel { state, count },
            prev_model: None,
        }
    }
}

impl From<RocketLauncherSamData> for RocketLauncherSamModel {
    fn from(data: RocketLauncherSamData) -> Self {
        RocketLauncherSamModel {
            model: data.model,
            prev_model: data.prev_model,
        }
    }
}
