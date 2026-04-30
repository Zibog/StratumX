use super::artifacts::ConveyorItem;

pub(crate) fn record_stage_timestamp(item: &mut ConveyorItem, metadata_key: &str) {
    item.set_metadata(metadata_key, unix_timestamp_secs());
}

fn unix_timestamp_secs() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}
