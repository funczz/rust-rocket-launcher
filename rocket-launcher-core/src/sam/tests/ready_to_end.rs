#[cfg(test)]
mod tests {

    use rust_sam::{SamAction, SamModelPresent, SamStateNextAction, SamStateRepresentation};

    use crate::{
        domain,
        sam::{
            action::{DecrementAction, FinishAction, StartAction},
            RocketLauncherSamData, RocketLauncherSamModel, RocketLauncherSamState,
        },
    };

    #[test]
    fn test() {
        let data = RocketLauncherSamData::new(domain::RocketLauncherState::Ready, Some(10u8));
        let mut rocket_launcher_sam_model =
            StartAction::execute(RocketLauncherSamModel::present, data).unwrap();
        assert_eq!(
            rocket_launcher_sam_model.model.state,
            domain::RocketLauncherState::Counting
        );
        assert_eq!(rocket_launcher_sam_model.model.count.unwrap(), 10u8);
        assert_eq!(
            RocketLauncherSamState::representation(rocket_launcher_sam_model.clone(), ()).unwrap().as_str(),
            "RocketLauncherSamModel { model: RocketLauncherModel { state: Counting, count: Some(10) }, prev_model: Some(RocketLauncherModel { state: Ready, count: Some(10) }) }"
        );
        for _ in 0..10 {
            let data = rocket_launcher_sam_model.into();
            rocket_launcher_sam_model =
                DecrementAction::execute(RocketLauncherSamModel::present, data).unwrap();
            rocket_launcher_sam_model =
                RocketLauncherSamState::next_action(rocket_launcher_sam_model).unwrap();
        }
        assert_eq!(
            rocket_launcher_sam_model.model.state,
            domain::RocketLauncherState::Launched
        );
        assert_eq!(rocket_launcher_sam_model.model.count.unwrap(), 0);
        assert_eq!(
            RocketLauncherSamState::representation(rocket_launcher_sam_model.clone(), ()).unwrap().as_str(),
            "RocketLauncherSamModel { model: RocketLauncherModel { state: Launched, count: Some(0) }, prev_model: Some(RocketLauncherModel { state: Counting, count: Some(0) }) }"
        );

        rocket_launcher_sam_model = FinishAction::execute(
            RocketLauncherSamModel::present,
            rocket_launcher_sam_model.into(),
        )
        .unwrap();
        assert_eq!(
            rocket_launcher_sam_model.model.state,
            domain::RocketLauncherState::End
        );

        assert_eq!(rocket_launcher_sam_model.model.count.unwrap(), 0);
    }
}
