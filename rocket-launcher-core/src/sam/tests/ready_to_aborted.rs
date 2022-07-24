#[cfg(test)]
mod tests {
    use rust_sam::{SamAction, SamModelPresent};

    use crate::{
        domain,
        sam::{action::AbortAction, RocketLauncherSamData, RocketLauncherSamModel},
    };

    #[test]
    fn abort_action() {
        let data = RocketLauncherSamData::new(domain::RocketLauncherState::Ready, None);
        let result = AbortAction::execute(RocketLauncherSamModel::present, data);
        match result {
            Ok(v) => assert_eq!(
                format!("{:?}", v),
                 "RocketLauncherSamModel { model: RocketLauncherModel { state: Aborted, count: None }, prev_model: Some(RocketLauncherModel { state: Ready, count: None }) }"
                ),
            Err(e) => panic!("{}", e.to_string()),
        }
    }
}
