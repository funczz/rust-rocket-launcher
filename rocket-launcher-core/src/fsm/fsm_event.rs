#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RocketLauncherEvent {
    START(u8),
    DECREMENT,
    LAUNCH,
    ABORT,
    FINISH,
}
