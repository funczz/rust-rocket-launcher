use crate::domain;

use super::RocketLauncherSamModel;

/**
 * SAM アクション入力データ
 */

#[derive(Debug, Clone)]
pub struct RocketLauncherSamData {
    pub model: domain::RocketLauncherModel,
    pub prev_model: Option<domain::RocketLauncherModel>,
}

impl RocketLauncherSamData {
    pub fn new(state: domain::RocketLauncherState, count: Option<u8>) -> Self {
        RocketLauncherSamData {
            model: domain::RocketLauncherModel { state, count },
            prev_model: None,
        }
    }
}

impl From<domain::RocketLauncherModel> for RocketLauncherSamData {
    fn from(model: domain::RocketLauncherModel) -> Self {
        RocketLauncherSamData {
            model: model,
            prev_model: None,
        }
    }
}

impl Into<domain::RocketLauncherModel> for RocketLauncherSamData {
    fn into(self) -> domain::RocketLauncherModel {
        domain::RocketLauncherModel {
            state: self.model.state,
            count: self.model.count,
        }
    }
}

impl From<RocketLauncherSamModel> for RocketLauncherSamData {
    fn from(model: RocketLauncherSamModel) -> Self {
        RocketLauncherSamData {
            model: model.model,
            prev_model: model.prev_model,
        }
    }
}
