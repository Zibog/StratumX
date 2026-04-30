use stratumx_tooling_l6_0_authority_core::{
    CertificationPipeline, ConveyorItem, ConveyorStage, CookPipeline, ImportPipeline,
    ToolingConveyor,
};

#[test]
fn import_pipeline_rejects_empty_source_path() {
    let mut pipeline = ImportPipeline::new();

    let item = pipeline
        .import("mat_001", "/assets/materials/concrete.mat")
        .unwrap();

    assert_eq!(item.asset_id, "mat_001");
    assert_eq!(item.current_stage, ConveyorStage::Imported);
    assert!(pipeline.import("mat_002", "").is_err());
}

#[test]
fn cook_pipeline_marks_item_as_cooked() {
    let mut pipeline = CookPipeline::new();

    let item = ConveyorItem::new("mat_001", "/assets/materials/concrete.mat");
    let cooked = pipeline.cook(item).unwrap();

    assert_eq!(cooked.current_stage, ConveyorStage::Cooked);
}

#[test]
fn certification_pipeline_requires_cooked_items() {
    let mut pipeline = CertificationPipeline::new();

    let item = ConveyorItem::new("mat_001", "/assets/materials/concrete.mat");
    assert!(pipeline.certify(item).is_err());

    let mut cooked = ConveyorItem::new("mat_001", "/assets/materials/concrete.mat");
    cooked.current_stage = ConveyorStage::Cooked;

    let certified = pipeline.certify(cooked).unwrap();
    assert_eq!(certified.current_stage, ConveyorStage::Certified);
    assert!(pipeline.is_certified("mat_001"));
}

#[test]
fn tooling_conveyor_processes_asset_through_all_stages() {
    let mut conveyor = ToolingConveyor::new();

    let certified = conveyor
        .process_asset("mat_001", "/assets/materials/concrete.mat")
        .unwrap();

    assert_eq!(certified.current_stage, ConveyorStage::Certified);
    assert_eq!(certified.asset_id, "mat_001");
}
