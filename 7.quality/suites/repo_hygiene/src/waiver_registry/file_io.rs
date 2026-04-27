use super::WaiverRegistry;
use std::fs;
use std::path::Path;

pub fn load_from_file(path: &Path) -> Result<WaiverRegistry, String> {
    let contents = fs::read_to_string(path)
        .map_err(|error| format!("Failed to read waiver registry file: {}", error))?;

    let registry: WaiverRegistry = toml::from_str(&contents)
        .map_err(|error| format!("Failed to parse waiver registry TOML: {}", error))?;

    validate_justifications(&registry)?;
    Ok(registry)
}

fn validate_justifications(registry: &WaiverRegistry) -> Result<(), String> {
    validate_entries("Line limit", &registry.line_limit_waivers)?;
    validate_entries("Allow attribute", &registry.allow_attr_waivers)?;
    validate_entries("Test-in-src", &registry.test_in_src_waivers)?;
    validate_entries("Execute bridge", &registry.execute_bridge_waivers)?;
    Ok(())
}

fn validate_entries(kind: &str, entries: &[super::WaiverEntry]) -> Result<(), String> {
    for entry in entries {
        if entry.justification.trim().is_empty() {
            return Err(format!(
                "{} waiver for '{}' is missing justification",
                kind, entry.path
            ));
        }
    }
    Ok(())
}
