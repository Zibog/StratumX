use repo_hygiene::{
    CacheRebuildabilityValidator, CodebaseState, OwnerInventory, OwnershipCompletenessValidator,
    PersistenceSeparationValidator, UiStateClassificationValidator,
};

fn main() {
    let exit_code = match run_validation() {
        Ok(()) => {
            println!("\n✓ All validation checks passed!");
            0
        }
        Err(violations) => {
            eprintln!("\n✗ Validation failed with {} violation(s)", violations);
            1
        }
    };

    std::process::exit(exit_code);
}

fn run_validation() -> Result<(), usize> {
    println!("State Ownership Validation Runner");
    println!("==================================\n");

    // Get the workspace root
    let root = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return Err(1);
        }
    };

    // Navigate to workspace root (we might be in a subdirectory)
    let workspace_root = find_workspace_root(&root).unwrap_or(root.clone());
    println!("Workspace root: {}\n", workspace_root.display());

    // Load owner inventory
    let inventory_path = workspace_root.join("7.quality/suites/repo_hygiene/owner_inventory.json");
    println!("Loading owner inventory from: {}", inventory_path.display());

    let inventory = match OwnerInventory::load_from_file(&inventory_path) {
        Ok(inv) => {
            println!("✓ Loaded {} inventory entries\n", inv.entries.len());
            inv
        }
        Err(e) => {
            eprintln!("✗ Failed to load owner inventory: {}", e);
            eprintln!("  Run 'cargo run --bin generate_owner_inventory' first");
            return Err(1);
        }
    };

    // Scan codebase
    println!("Scanning codebase for state fields...");
    let codebase_state = match CodebaseState::scan_from_directory(&workspace_root) {
        Ok(state) => {
            println!("✓ Found {} state fields\n", state.state_fields.len());
            state
        }
        Err(e) => {
            eprintln!("✗ Failed to scan codebase: {}", e);
            return Err(1);
        }
    };

    let mut total_violations = 0;

    // Run ownership completeness validation
    println!("Running ownership completeness validation...");
    println!("--------------------------------------------");
    match run_ownership_validation(&inventory, &codebase_state) {
        Ok(()) => println!("✓ Ownership validation passed\n"),
        Err(count) => {
            total_violations += count;
            println!();
        }
    }

    // Run persistence separation validation
    println!("Running persistence separation validation...");
    println!("--------------------------------------------");
    match run_persistence_validation(&inventory, &codebase_state) {
        Ok(()) => println!("✓ Persistence validation passed\n"),
        Err(count) => {
            total_violations += count;
            println!();
        }
    }

    // Run cache rebuildability validation
    println!("Running cache rebuildability validation...");
    println!("--------------------------------------------");
    match run_cache_validation(&codebase_state) {
        Ok(()) => println!("✓ Cache validation passed\n"),
        Err(count) => {
            total_violations += count;
            println!();
        }
    }

    // Run UI state classification validation
    println!("Running UI state classification validation...");
    println!("--------------------------------------------");
    match run_ui_state_validation(&inventory, &codebase_state) {
        Ok(()) => println!("✓ UI state validation passed\n"),
        Err(count) => {
            total_violations += count;
            println!();
        }
    }

    if total_violations > 0 {
        Err(total_violations)
    } else {
        Ok(())
    }
}

fn run_ownership_validation(
    inventory: &OwnerInventory,
    codebase_state: &CodebaseState,
) -> Result<(), usize> {
    let validator = OwnershipCompletenessValidator::new(inventory.clone());

    match validator.validate(codebase_state) {
        Ok(()) => Ok(()),
        Err(violations) => {
            eprintln!("✗ Found {} ownership violation(s):", violations.len());
            for (i, violation) in violations.iter().enumerate() {
                eprintln!("\n  {}. {}", i + 1, violation.message);
                eprintln!("     Entity: {}", violation.entity_name);
                if !violation.file_path.is_empty() {
                    eprintln!("     File: {}", violation.file_path);
                }
                eprintln!("     Type: {:?}", violation.violation_type);
            }
            Err(violations.len())
        }
    }
}

fn run_persistence_validation(
    inventory: &OwnerInventory,
    codebase_state: &CodebaseState,
) -> Result<(), usize> {
    let validator = PersistenceSeparationValidator::new(inventory.clone());

    match validator.validate(codebase_state) {
        Ok(()) => Ok(()),
        Err(violations) => {
            eprintln!("✗ Found {} persistence violation(s):", violations.len());
            for (i, violation) in violations.iter().enumerate() {
                eprintln!("\n  {}. {}", i + 1, violation.message);
                eprintln!("     File: {}", violation.file_path);
                eprintln!("     Type: {:?}", violation.violation_type);
            }
            Err(violations.len())
        }
    }
}

fn run_cache_validation(codebase_state: &CodebaseState) -> Result<(), usize> {
    let validator = CacheRebuildabilityValidator::new();

    match validator.validate(codebase_state) {
        Ok(()) => Ok(()),
        Err(violations) => {
            eprintln!("✗ Found {} cache violation(s):", violations.len());
            for (i, violation) in violations.iter().enumerate() {
                eprintln!("\n  {}. {}", i + 1, violation.message);
                eprintln!("     Cache: {}", violation.cache_name);
                eprintln!("     File: {}", violation.file_path);
                eprintln!("     Type: {:?}", violation.violation_type);
            }
            Err(violations.len())
        }
    }
}

fn run_ui_state_validation(
    inventory: &OwnerInventory,
    codebase_state: &CodebaseState,
) -> Result<(), usize> {
    let validator = UiStateClassificationValidator::new(inventory.clone());

    match validator.validate(codebase_state) {
        Ok(()) => Ok(()),
        Err(violations) => {
            eprintln!("✗ Found {} UI state violation(s):", violations.len());
            for (i, violation) in violations.iter().enumerate() {
                eprintln!("\n  {}. {}", i + 1, violation.message);
                eprintln!("     Component: {}", violation.component_name);
                eprintln!("     Field: {}", violation.field_name);
                eprintln!("     File: {}", violation.file_path);
                eprintln!("     Type: {:?}", violation.violation_type);
            }
            Err(violations.len())
        }
    }
}

/// Find the workspace root by looking for Cargo.toml in parent directories
fn find_workspace_root(start: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut current = start;

    // Look for a directory containing both Cargo.toml and specific workspace markers
    loop {
        // Check if this directory looks like the workspace root
        let cargo_toml = current.join("Cargo.toml");
        let quality_dir = current.join("7.quality");

        if cargo_toml.exists() && quality_dir.exists() {
            return Some(current.to_path_buf());
        }

        // Move up to parent directory
        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}
