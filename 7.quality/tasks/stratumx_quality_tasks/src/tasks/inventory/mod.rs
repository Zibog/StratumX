use super::common::QualityContext;
use std::fs;

mod classification;
mod cleanliness;
mod discovery;
mod file_size_discipline;
mod models;
mod rendering;
mod report;
mod stats;
mod test_placement;

pub fn run(ctx: &QualityContext) -> Result<(), String> {
    println!("Starting inventory generation...");

    // Check root cleanliness first
    let cleanliness_violations = cleanliness::check_root_cleanliness(&ctx.repo_root)?;
    println!(
        "{}",
        cleanliness::format_violations(&cleanliness_violations)
    );

    if !cleanliness_violations.is_empty() {
        return Err(format!(
            "Root cleanliness check failed with {} violations",
            cleanliness_violations.len()
        ));
    }

    // Check file size discipline
    let file_size_violations = file_size_discipline::check_file_size_discipline(&ctx.repo_root)?;
    println!(
        "{}",
        file_size_discipline::format_violations(&file_size_violations)
    );

    // Count unjustified violations
    let unjustified_count = file_size_violations
        .iter()
        .filter(|v| !v.has_justification)
        .count();
    if unjustified_count > 0 {
        return Err(format!(
            "File size discipline check failed with {} unjustified violations",
            unjustified_count
        ));
    }

    // Check test placement (heavy tests must be in 7.quality)
    let test_placement_violations = test_placement::check_test_placement(&ctx.repo_root)?;
    println!(
        "{}",
        test_placement::format_violations(&test_placement_violations)
    );

    if !test_placement_violations.is_empty() {
        return Err(format!(
            "Test placement check failed with {} misplaced heavy test(s)",
            test_placement_violations.len()
        ));
    }

    let docs_dir = report::ensure_docs_dir(ctx)?;
    println!("✓ Ensured 7.quality/docs/ directory exists");

    let suites_dir = ctx.repo_root.join("7.quality").join("suites");
    let inventory = discovery::scan_test_files(&suites_dir)?;
    println!("✓ Scanned {} test files", inventory.len());

    let inventory_content = rendering::generate_inventory_markdown(&inventory);
    let inventory_path = docs_dir.join("INVENTORY.md");
    fs::write(&inventory_path, inventory_content)
        .map_err(|e| format!("Failed to write INVENTORY.md: {}", e))?;
    println!("✓ Generated INVENTORY.md at {:?}", inventory_path);

    Ok(())
}
