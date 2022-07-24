use std::{error, fmt};

use crate::{domain, SendCommonError};

/**
 * Rocket Launcher SAM エラー
 */

// Rocket Launcher SAM エラー: 構造体
#[derive(Debug)]
pub struct RocketLauncherSamError {
    pub action: String,
    pub model: domain::RocketLauncherModel,
    pub message: String,
}

// Rocket Launcher SAM エラー: SendCommonError 実装
impl SendCommonError for RocketLauncherSamError {}

// Rocket Launcher SAM エラー: fmt::Display 実装
impl fmt::Display for RocketLauncherSamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} (action: {}, model: {:?})",
            self.message, self.action, self.model
        )
    }
}

// Rocket Launcher SAM エラー: error::Error 実装
impl error::Error for RocketLauncherSamError {}
