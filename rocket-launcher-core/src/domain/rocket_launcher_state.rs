#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RocketLauncherState {
    Aborted,
    Counting,
    End,
    Launched,
    Ready,
}
