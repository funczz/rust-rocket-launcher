#[cfg(test)]
mod tests {
    use rust_sam::{SamAction, SamModelPresent};

    use crate::{
        domain,
        sam::{action::StartAction, RocketLauncherSamData, RocketLauncherSamModel},
    };

    #[test]
    fn count_error_none() {
        let data = RocketLauncherSamData::new(domain::RocketLauncherState::Ready, None);
        let result = StartAction::execute(RocketLauncherSamModel::present, data);
        match result {
            Ok(v) => panic!("{:?}", v),
            Err(e) => assert_eq!(e.to_string().as_str(), "count error: None (action: StartAction, model: RocketLauncherModel { state: Ready, count: None })"),
        }
    }

    #[test]
    fn event_not_found_start() {
        let data = RocketLauncherSamData::new(domain::RocketLauncherState::Counting, Some(10u8));
        let result = StartAction::execute(RocketLauncherSamModel::present, data);
        match result {
            Ok(v) => panic!("{:?}", v),
            Err(e) => assert_eq!(e.to_string().as_str(), "state error. (action: StartAction, model: RocketLauncherModel { state: Counting, count: Some(10) })"),
        }
    }
}
