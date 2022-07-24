#[cfg(test)]
mod tests {

    use crate::sam;

    //
    // SAM ステートの ::representation 実装
    //
    impl rust_sam::SamStateRepresentation for sam::RocketLauncherSamState {
        type Error = sam::RocketLauncherSamError;
        type RepresentationData = ();
        type Result = String;
        type SamModel = sam::RocketLauncherSamModel;

        fn representation(
            model: sam::RocketLauncherSamModel,
            _representation_data: (),
        ) -> Result<String, sam::RocketLauncherSamError> {
            Ok(format!("{:?}", model))
        }
    }
}
