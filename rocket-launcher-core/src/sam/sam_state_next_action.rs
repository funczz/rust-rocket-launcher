use rust_sam::{SamAction, SamModelPresent, SamStateNextAction};

use crate::domain;

use super::{
    action::LaunchAction, sam_model::RocketLauncherSamModel, RocketLauncherSamData,
    RocketLauncherSamError, RocketLauncherSamState,
};

/**
 * SAM ステート ::next_action 実装
 */

impl SamStateNextAction for RocketLauncherSamState {
    type Error = RocketLauncherSamError;
    type SamModel = RocketLauncherSamModel;

    fn next_action_predicate(
        data: rust_sam::NextActionData<RocketLauncherSamModel>,
    ) -> Result<rust_sam::NextActionData<RocketLauncherSamModel>, RocketLauncherSamError> {
        let model = match data {
            rust_sam::NextActionData::Continue { model } => model,
            rust_sam::NextActionData::Terminate { model: _ } => return Ok(data),
        };
        let new = match (&model.model.state, &model.model.count) {
            // launch
            (domain::RocketLauncherState::Counting, Some(v)) if v == &0u8 => {
                let data = RocketLauncherSamData::from(model.clone().model);
                match LaunchAction::execute(RocketLauncherSamModel::present, data.clone()) {
                    Ok(v) => rust_sam::NextActionData::new_continue(v),
                    Err(e) => {
                        return Err(RocketLauncherSamError {
                            action: String::from("next_action(LaunchAction)"),
                            model: data.into(),
                            message: e.to_string(),
                        })
                    }
                }
            }
            // else
            (_, _) => rust_sam::NextActionData::new_terminate(model),
        };
        Ok(new)
    }
}
