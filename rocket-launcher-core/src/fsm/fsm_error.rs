use std::{error, fmt};

use crate::{domain, fsm, SendCommonError};

/**
 * Rocket Launcher Fsm エラー
 */

// Rocket Launcher Fsm エラー: 構造体
#[derive(Debug)]
pub enum RocketLauncherFsmError {
    EventNotFoundError {
        event: fsm::RocketLauncherEvent,
    },
    Error {
        state: Box<
            dyn fsm::FsmState<
                Event = fsm::RocketLauncherEvent,
                Ctx = domain::RocketLauncherModel,
                Error = RocketLauncherFsmError,
            >,
        >,
        event: fsm::RocketLauncherEvent,
        message: String,
    },
}

// Rocket Launcher Fsm エラー: SendCommonError 実装
impl SendCommonError for RocketLauncherFsmError {}

// Rocket Launcher Fsm エラー: fmt::Display 実装
impl fmt::Display for RocketLauncherFsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::EventNotFoundError { event } => {
                write!(f, "event not found: {:?}", event)
            }
            Self::Error {
                state,
                event,
                message,
            } => {
                write!(
                    f,
                    "state: {:?}, event: {:?}, message: {}",
                    state, event, message
                )
            }
        }
    }
}

// Rocket Launcher Fsm エラー: error::Error 実装
impl error::Error for RocketLauncherFsmError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
