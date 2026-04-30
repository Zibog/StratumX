// Gate: Crate status ledger must exist

use std::path::Path;

#[test]
fn crate_status_ledger_exists() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

    assert!(
        ledger_path.exists(),
        "Crate status ledger must exist at 7.quality/inventory/crate_status_ledger.md"
    );

    let content =
        std::fs::read_to_string(&ledger_path).expect("Failed to read crate_status_ledger.md");

    assert!(
        content.contains("ACTIVE_CORE"),
        "Ledger must contain ACTIVE_CORE status"
    );

    assert!(
        content.contains("FUTURE_STUB"),
        "Ledger must contain FUTURE_STUB status"
    );

    assert!(
        content.contains("l8.8-interaction-routing-system"),
        "Ledger must classify l8.8-interaction-routing-system"
    );
}
