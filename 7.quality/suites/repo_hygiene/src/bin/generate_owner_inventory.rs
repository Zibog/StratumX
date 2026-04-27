use repo_hygiene::{CodebaseState, OwnerInventory, OwnerInventoryEntry, StateClassification};

fn main() -> Result<(), String> {
    println!("Scanning codebase for state fields...");

    // Get the workspace root (3 levels up from the binary location)
    let root =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    println!("Scanning from root: {}", root.display());

    let codebase_state = CodebaseState::scan_from_directory(&root)?;

    println!("Found {} state fields", codebase_state.state_fields.len());

    // Create initial inventory with manual classification
    let mut inventory = OwnerInventory::new();
    let mut seen_entities = std::collections::HashSet::new();

    // Add entries based on scanned fields
    for field in &codebase_state.state_fields {
        let entity_name = format!("{}::{}", field.owner_type, field.name);

        // Skip duplicates
        if seen_entities.contains(&entity_name) {
            continue;
        }
        seen_entities.insert(entity_name.clone());

        // Determine target owner and classification based on field name and owner type
        let (target_owner, classification) = classify_field(&field.name, &field.owner_type);

        let entry = OwnerInventoryEntry {
            entity_name: format!("{}::{}", field.owner_type, field.name),
            current_path: field.file_path.clone(),
            current_owner: field.owner_type.clone(),
            target_owner: target_owner.to_string(),
            classification,
            can_mutate: vec![format!("{}Service", target_owner)],
            can_read: vec!["QueryLayer".to_string()],
            publishes_changes: Some("EventBus".to_string()),
            rebuilds_cache: if classification == StateClassification::Derived {
                Some(format!("{}Cache", field.owner_type))
            } else {
                None
            },
        };

        inventory.entries.push(entry);
    }

    // Save to JSON file
    let output_path = root.join("7.quality/suites/repo_hygiene/owner_inventory.json");
    inventory.save_to_file(&output_path)?;

    println!("Owner inventory saved to {}", output_path.display());

    // Also save as markdown
    let md_path = root.join("7.quality/suites/repo_hygiene/OWNER_INVENTORY.md");
    inventory.save_to_file(&md_path)?;

    println!("Owner inventory saved to {}", md_path.display());

    Ok(())
}

fn classify_field(field_name: &str, owner_type: &str) -> (&'static str, StateClassification) {
    // Classify based on owner type and field name patterns
    match owner_type {
        "ProjectState" => ("ProjectOwner", StateClassification::Persistable),
        "WorkspaceState" => ("WorkspaceOwner", StateClassification::Persistable),
        "WorldState" => ("WorldOwner", StateClassification::Persistable),
        "DiagnosticsState" => ("DiagnosticsOwner", StateClassification::Persistable),
        _ => {
            // Check field name patterns for classification
            if field_name.contains("hover")
                || field_name.contains("drag")
                || field_name.contains("focus")
                || field_name.contains("selected")
                || field_name.contains("search")
                || field_name.contains("scroll")
            {
                ("UiTransientState", StateClassification::Transient)
            } else if field_name.contains("cache")
                || field_name.contains("filtered")
                || field_name.contains("summary")
                || field_name.contains("view")
            {
                ("CacheLayer", StateClassification::Derived)
            } else {
                // Default to persistable for domain services
                ("WorldOwner", StateClassification::Persistable)
            }
        }
    }
}
