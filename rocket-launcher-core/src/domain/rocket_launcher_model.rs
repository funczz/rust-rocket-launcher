use super::RocketLauncherState;

#[derive(Debug, PartialEq)]
pub struct RocketLauncherModel {
    pub state: RocketLauncherState,
    pub count: Option<u8>,
}

impl Clone for RocketLauncherModel {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            count: self.count.clone(),
        }
    }
}
