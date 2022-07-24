use rust_sam::SamModelPresent;

use super::{
    sam_data::RocketLauncherSamData, sam_model::RocketLauncherSamModel, RocketLauncherSamError,
};

/**
 * SAM モデル ::present 実装
 */
impl SamModelPresent for RocketLauncherSamModel {
    type ActionData = RocketLauncherSamData;
    type Error = RocketLauncherSamError;

    fn present(
        data: RocketLauncherSamData,
    ) -> Result<RocketLauncherSamModel, RocketLauncherSamError> {
        //
        // 永続化の処理はここに記述する。
        //
        Ok(RocketLauncherSamModel::from(data))
    }
}
