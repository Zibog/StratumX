mod common;
use common::*;

fn run_acoustics_case(source_count: usize, stream_upload_bytes: usize) -> AcousticReceipt {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 64,
    });
    synthesize_receipt(
        &service,
        &mut transfer,
        AcousticsRequest {
            source_count,
            stream_upload_bytes,
        },
        MaterialId(1),
    )
    .unwrap()
}

macro_rules! acoustics_case {
    ($name:ident, $source_count:expr, $stream_upload_bytes:expr) => {
        #[test]
        fn $name() {
            let receipt = run_acoustics_case($source_count, $stream_upload_bytes);
            assert_eq!(receipt.propagated_sources, $source_count);
        }
    };
}

include!("acoustics_matrix/cases_01.rs");
include!("acoustics_matrix/cases_02.rs");
include!("acoustics_matrix/cases_03.rs");
include!("acoustics_matrix/cases_04.rs");
include!("acoustics_matrix/cases_05.rs");
