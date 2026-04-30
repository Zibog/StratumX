use super::steps::ConveyorStage;

pub(crate) fn empty_source_path() -> String {
    "Source path cannot be empty".to_string()
}

pub(crate) fn cannot_cook_item_with_import_errors() -> String {
    "Cannot cook item with import errors".to_string()
}

pub(crate) fn cannot_certify_stage(stage: ConveyorStage) -> String {
    format!("Cannot certify item in {:?} stage - must be Cooked", stage)
}

pub(crate) fn cannot_certify_item_with_errors() -> String {
    "Cannot certify item with errors".to_string()
}
