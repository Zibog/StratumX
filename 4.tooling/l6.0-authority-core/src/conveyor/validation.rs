use super::artifacts::ConveyorItem;
use super::failure::{
    cannot_certify_item_with_errors, cannot_certify_stage, cannot_cook_item_with_import_errors,
    empty_source_path,
};
use super::steps::ConveyorStage;

pub(crate) fn validate_source_path(source_path: &str) -> Result<(), String> {
    if source_path.is_empty() {
        Err(empty_source_path())
    } else {
        Ok(())
    }
}

pub(crate) fn ensure_cookable(item: &ConveyorItem) -> Result<(), String> {
    if item.errors.iter().any(|error| error.contains("import")) {
        Err(cannot_cook_item_with_import_errors())
    } else {
        Ok(())
    }
}

pub(crate) fn ensure_certifiable(item: &ConveyorItem) -> Result<(), String> {
    if item.current_stage != ConveyorStage::Cooked {
        return Err(cannot_certify_stage(item.current_stage));
    }

    if !item.errors.is_empty() {
        return Err(cannot_certify_item_with_errors());
    }

    Ok(())
}
