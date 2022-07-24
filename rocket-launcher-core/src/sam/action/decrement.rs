use rust_sam::SamAction;

use crate::{
    domain, fsm,
    sam::{RocketLauncherSamData, RocketLauncherSamError, RocketLauncherSamModel},
};

/**
 * デクリメント SAM アクション
 */

#[derive(Debug, Clone)]
pub struct DecrementAction;

// デクリメント SAM アクション ::execute 実装
impl SamAction for DecrementAction {
    type ActionData = RocketLauncherSamData;
    type Error = RocketLauncherSamError;
    type SamModel = RocketLauncherSamModel;

    fn execute(
        present: fn(
            RocketLauncherSamData,
        ) -> Result<RocketLauncherSamModel, RocketLauncherSamError>,
        data: RocketLauncherSamData,
    ) -> Result<RocketLauncherSamModel, RocketLauncherSamError> {
        let model: domain::RocketLauncherModel = data.into();
        let state: Box<
            dyn rust_fsm::FsmState<
                Event = fsm::RocketLauncherEvent,
                Ctx = domain::RocketLauncherModel,
                Error = fsm::RocketLauncherFsmError,
            >,
        > = model.clone().state.into();
        let result = state.fire(fsm::RocketLauncherEvent::DECREMENT, model.clone());
        let new_model = match result {
            Ok((_, v)) => v,
            Err(e) => {
                return Err(RocketLauncherSamError {
                    action: format!("{:?}", Self),
                    model: model.into(),
                    message: e.to_string(),
                })
            }
        };
        let data = RocketLauncherSamData {
            model: new_model,
            prev_model: Some(model),
        };
        present(data)
    }
}
