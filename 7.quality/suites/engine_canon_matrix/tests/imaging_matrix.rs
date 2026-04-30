mod common;
use common::*;

fn run_imaging_case(upload_bytes: usize) -> ImagingReceipt {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 8,
        max_upload_bytes: 64,
    });
    render_receipt(
        &service,
        &mut transfer,
        ImagingRequest {
            render_target_id: 1,
            view_region: (0, 0, 0),
            upload_bytes,
        },
        MaterialId(1),
    )
    .unwrap()
}

macro_rules! imaging_case {
    ($name:ident, $upload_bytes:expr) => {
        #[test]
        fn $name() {
            let receipt = run_imaging_case($upload_bytes);
            assert_eq!(receipt.upload_bytes, $upload_bytes);
        }
    };
}

include!("imaging_matrix/cases_01.rs");
include!("imaging_matrix/cases_02.rs");
include!("imaging_matrix/cases_03.rs");
include!("imaging_matrix/cases_04.rs");
include!("imaging_matrix/cases_05.rs");
