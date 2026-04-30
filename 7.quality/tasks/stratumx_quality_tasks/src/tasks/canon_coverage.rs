use super::common::QualityContext;
use serde_json::json;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Represents a canonical package that should have code coverage
#[derive(Debug, Clone)]
struct CanonicalPackage {
    name: String,
    layer: Layer,
}

#[derive(Debug, Clone, PartialEq)]
enum Layer {
    Engine,
    Sdk,
    Tooling,
    Editor,
}

/// Represents a heavy domain that needs full coverage across all layers
#[derive(Debug, Clone)]
struct HeavyDomain {
    name: String,
    has_engine: bool,
    has_sdk: bool,
    has_tooling: bool,
    has_editor: bool,
}

/// Represents a vertical slice that should be closed end-to-end
#[derive(Debug, Clone)]
struct VerticalSlice {
    name: String,
    has_engine: bool,
    has_sdk: bool,
    has_tooling: bool,
    has_editor: bool,
}

pub fn run(ctx: &QualityContext) -> Result<(), String> {
    println!("Calculating canon coverage metrics...");

    // Get all actual packages in the workspace
    let actual_packages = get_actual_packages(ctx)?;

    // Define canonical packages based on stack map
    let canonical_packages = define_canonical_packages();

    // Calculate percentage of canonical packages with code owner
    let packages_with_code = calculate_packages_with_code(&canonical_packages, &actual_packages);

    // Define heavy domains
    let heavy_domains = define_heavy_domains();

    // Calculate percentage of heavy domains with full coverage
    let domains_complete = calculate_heavy_domains_complete(&heavy_domains, &actual_packages);

    // Define vertical slices
    let vertical_slices = define_vertical_slices();

    // Calculate percentage of vertical slices closed end-to-end
    let slices_closed = calculate_vertical_slices_closed(&vertical_slices, &actual_packages);

    // Generate list of missing components
    let missing_components = generate_missing_components(
        &canonical_packages,
        &heavy_domains,
        &vertical_slices,
        &actual_packages,
    );

    // Generate markdown report
    let markdown = format!(
        "# Canon Coverage Metrics\n\n\
        ## Summary\n\n\
        - **Canonical packages with code owner**: {:.1}% ({}/{})\n\
        - **Heavy domains with full coverage**: {:.1}% ({}/{})\n\
        - **Vertical slices closed end-to-end**: {:.1}% ({}/{})\n\n\
        ## Missing Components\n\n{}\n\n\
        ## Details\n\n\
        ### Canonical Packages\n{}\n\n\
        ### Heavy Domains\n{}\n\n\
        ### Vertical Slices\n{}\n",
        packages_with_code.percentage,
        packages_with_code.count,
        packages_with_code.total,
        domains_complete.percentage,
        domains_complete.count,
        domains_complete.total,
        slices_closed.percentage,
        slices_closed.count,
        slices_closed.total,
        format_missing_components(&missing_components),
        format_canonical_packages(&canonical_packages, &actual_packages),
        format_heavy_domains(&heavy_domains),
        format_vertical_slices(&vertical_slices),
    );

    ctx.write_text("metrics/canon-coverage.md", &markdown)?;

    // Generate JSON report
    let json = json!({
        "canonical_packages_with_code_percentage": packages_with_code.percentage,
        "canonical_packages_with_code_count": packages_with_code.count,
        "canonical_packages_total": packages_with_code.total,
        "heavy_domains_complete_percentage": domains_complete.percentage,
        "heavy_domains_complete_count": domains_complete.count,
        "heavy_domains_total": domains_complete.total,
        "vertical_slices_closed_percentage": slices_closed.percentage,
        "vertical_slices_closed_count": slices_closed.count,
        "vertical_slices_total": slices_closed.total,
        "missing_components": missing_components,
    });

    ctx.write_text(
        "metrics/canon-coverage.json",
        &serde_json::to_string_pretty(&json).map_err(|err| err.to_string())?,
    )?;

    println!("\nCanon Coverage Summary:");
    println!(
        "  Canonical packages with code: {:.1}% ({}/{})",
        packages_with_code.percentage, packages_with_code.count, packages_with_code.total
    );
    println!(
        "  Heavy domains complete: {:.1}% ({}/{})",
        domains_complete.percentage, domains_complete.count, domains_complete.total
    );
    println!(
        "  Vertical slices closed: {:.1}% ({}/{})",
        slices_closed.percentage, slices_closed.count, slices_closed.total
    );

    if !missing_components.is_empty() {
        println!("\n  Missing components: {}", missing_components.len());
    }

    ctx.write_task_summary("canon-coverage", &[], None)
}

struct CoverageMetric {
    percentage: f64,
    count: usize,
    total: usize,
}

fn get_actual_packages(ctx: &QualityContext) -> Result<HashSet<String>, String> {
    let mut packages = HashSet::new();

    // Scan each layer directory for Cargo.toml files
    for layer in &["2.engine", "3.sdk", "4.tooling", "5.editor"] {
        let layer_path = ctx.repo_root.join(layer);
        if layer_path.exists() {
            scan_packages(&layer_path, &mut packages);
        }
    }

    Ok(packages)
}

fn scan_packages(dir: &Path, packages: &mut HashSet<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Check if this directory has a Cargo.toml
                let cargo_toml = path.join("Cargo.toml");
                if cargo_toml.exists() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        packages.insert(name.to_string());
                    }
                }
                // Recursively scan subdirectories
                scan_packages(&path, packages);
            }
        }
    }
}

fn define_canonical_packages() -> Vec<CanonicalPackage> {
    vec![
        // Engine layer (L-0 through L4)
        CanonicalPackage {
            name: "l-0.05-world-region".to_string(),
            layer: Layer::Engine,
        },
        CanonicalPackage {
            name: "l-0.1-world-spatial".to_string(),
            layer: Layer::Engine,
        },
        CanonicalPackage {
            name: "l-0.2-ecs-assembly".to_string(),
            layer: Layer::Engine,
        },
        CanonicalPackage {
            name: "l-0.3-ecs-query".to_string(),
            layer: Layer::Engine,
        },
        CanonicalPackage {
            name: "l-0.4-ecs-registry".to_string(),
            layer: Layer::Engine,
        },
        CanonicalPackage {
            name: "l0.5-shared-world-property-substrate".to_string(),
            layer: Layer::Engine,
        },
        // SDK layer (L5)
        CanonicalPackage {
            name: "l5.0-link-ingress-packets".to_string(),
            layer: Layer::Sdk,
        },
        CanonicalPackage {
            name: "l5.2-link-egress-observations".to_string(),
            layer: Layer::Sdk,
        },
        CanonicalPackage {
            name: "l5.9-legality-gates".to_string(),
            layer: Layer::Sdk,
        },
        // Tooling layer (L6, L7)
        CanonicalPackage {
            name: "l6.0-authority-core".to_string(),
            layer: Layer::Tooling,
        },
        CanonicalPackage {
            name: "l6.1-command-envelopes".to_string(),
            layer: Layer::Tooling,
        },
        // Editor layer (L8, L9, L10)
        CanonicalPackage {
            name: "l7.0-editor-command-spine".to_string(),
            layer: Layer::Editor,
        },
        CanonicalPackage {
            name: "l8.0-editor-shell".to_string(),
            layer: Layer::Editor,
        },
        CanonicalPackage {
            name: "l9.0-world-authoring-suite".to_string(),
            layer: Layer::Editor,
        },
    ]
}

fn define_heavy_domains() -> Vec<HeavyDomain> {
    vec![
        HeavyDomain {
            name: "World Scale".to_string(),
            has_engine: true,
            has_sdk: true,
            has_tooling: true,
            has_editor: true,
        },
        HeavyDomain {
            name: "Material System".to_string(),
            has_engine: false,
            has_sdk: true,
            has_tooling: true,
            has_editor: true,
        },
        HeavyDomain {
            name: "Terrain System".to_string(),
            has_engine: false,
            has_sdk: true,
            has_tooling: true,
            has_editor: true,
        },
    ]
}

fn define_vertical_slices() -> Vec<VerticalSlice> {
    vec![
        VerticalSlice {
            name: "World-scale geodesy and coordinates".to_string(),
            has_engine: true,
            has_sdk: true,
            has_tooling: true,
            has_editor: true,
        },
        VerticalSlice {
            name: "Physical substrate (terrain, water, atmosphere)".to_string(),
            has_engine: false,
            has_sdk: true,
            has_tooling: true,
            has_editor: true,
        },
        VerticalSlice {
            name: "Living runtime (entities, AI, animation)".to_string(),
            has_engine: false,
            has_sdk: false,
            has_tooling: false,
            has_editor: false,
        },
        VerticalSlice {
            name: "Photoreal proof (rendering, lighting, materials)".to_string(),
            has_engine: false,
            has_sdk: true,
            has_tooling: true,
            has_editor: true,
        },
        VerticalSlice {
            name: "Audio reality (spatial audio, propagation)".to_string(),
            has_engine: false,
            has_sdk: false,
            has_tooling: true,
            has_editor: true,
        },
        VerticalSlice {
            name: "Animation and procedural interaction".to_string(),
            has_engine: false,
            has_sdk: false,
            has_tooling: false,
            has_editor: true,
        },
        VerticalSlice {
            name: "Fur/cloth/wetness simulation".to_string(),
            has_engine: false,
            has_sdk: false,
            has_tooling: false,
            has_editor: false,
        },
        VerticalSlice {
            name: "Release/capture/freeze workflows".to_string(),
            has_engine: false,
            has_sdk: false,
            has_tooling: true,
            has_editor: true,
        },
    ]
}

fn calculate_packages_with_code(
    canonical: &[CanonicalPackage],
    actual: &HashSet<String>,
) -> CoverageMetric {
    let total = canonical.len();
    let count = canonical
        .iter()
        .filter(|pkg| actual.contains(&pkg.name))
        .count();
    let percentage = if total > 0 {
        (count as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    CoverageMetric {
        percentage,
        count,
        total,
    }
}

fn calculate_heavy_domains_complete(
    domains: &[HeavyDomain],
    _actual: &HashSet<String>,
) -> CoverageMetric {
    let total = domains.len();
    let count = domains
        .iter()
        .filter(|d| d.has_engine && d.has_sdk && d.has_tooling && d.has_editor)
        .count();
    let percentage = if total > 0 {
        (count as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    CoverageMetric {
        percentage,
        count,
        total,
    }
}

fn calculate_vertical_slices_closed(
    slices: &[VerticalSlice],
    _actual: &HashSet<String>,
) -> CoverageMetric {
    let total = slices.len();
    let count = slices
        .iter()
        .filter(|s| s.has_engine && s.has_sdk && s.has_tooling && s.has_editor)
        .count();
    let percentage = if total > 0 {
        (count as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    CoverageMetric {
        percentage,
        count,
        total,
    }
}

fn generate_missing_components(
    canonical: &[CanonicalPackage],
    domains: &[HeavyDomain],
    slices: &[VerticalSlice],
    actual: &HashSet<String>,
) -> Vec<serde_json::Value> {
    let mut missing = Vec::new();

    // Missing canonical packages
    for pkg in canonical {
        if !actual.contains(&pkg.name) {
            missing.push(json!({
                "type": "canonical_package",
                "name": pkg.name,
                "layer": format!("{:?}", pkg.layer),
                "status": "missing",
            }));
        }
    }

    // Incomplete heavy domains
    for domain in domains {
        let mut missing_layers = Vec::new();
        if !domain.has_engine {
            missing_layers.push("engine");
        }
        if !domain.has_sdk {
            missing_layers.push("sdk");
        }
        if !domain.has_tooling {
            missing_layers.push("tooling");
        }
        if !domain.has_editor {
            missing_layers.push("editor");
        }

        if !missing_layers.is_empty() {
            missing.push(json!({
                "type": "heavy_domain",
                "name": domain.name,
                "missing_layers": missing_layers,
            }));
        }
    }

    // Incomplete vertical slices
    for slice in slices {
        let mut missing_layers = Vec::new();
        if !slice.has_engine {
            missing_layers.push("engine");
        }
        if !slice.has_sdk {
            missing_layers.push("sdk");
        }
        if !slice.has_tooling {
            missing_layers.push("tooling");
        }
        if !slice.has_editor {
            missing_layers.push("editor");
        }

        if !missing_layers.is_empty() {
            missing.push(json!({
                "type": "vertical_slice",
                "name": slice.name,
                "missing_layers": missing_layers,
            }));
        }
    }

    missing
}

fn format_missing_components(missing: &[serde_json::Value]) -> String {
    if missing.is_empty() {
        return "No missing components! 🎉".to_string();
    }

    let mut output = String::new();
    for component in missing {
        let comp_type = component["type"].as_str().unwrap_or("unknown");
        let name = component["name"].as_str().unwrap_or("unknown");

        match comp_type {
            "canonical_package" => {
                let layer = component["layer"].as_str().unwrap_or("unknown");
                output.push_str(&format!("- **Canonical Package**: {} ({})\n", name, layer));
            }
            "heavy_domain" => {
                let missing_layers = component["missing_layers"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .unwrap_or_default();
                output.push_str(&format!(
                    "- **Heavy Domain**: {} (missing: {})\n",
                    name, missing_layers
                ));
            }
            "vertical_slice" => {
                let missing_layers = component["missing_layers"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .unwrap_or_default();
                output.push_str(&format!(
                    "- **Vertical Slice**: {} (missing: {})\n",
                    name, missing_layers
                ));
            }
            _ => {}
        }
    }

    output
}

fn format_canonical_packages(canonical: &[CanonicalPackage], actual: &HashSet<String>) -> String {
    let mut output = String::new();
    for pkg in canonical {
        let status = if actual.contains(&pkg.name) {
            "✓"
        } else {
            "✗"
        };
        output.push_str(&format!("{} {} ({:?})\n", status, pkg.name, pkg.layer));
    }
    output
}

fn format_heavy_domains(domains: &[HeavyDomain]) -> String {
    let mut output = String::new();
    for domain in domains {
        let complete =
            domain.has_engine && domain.has_sdk && domain.has_tooling && domain.has_editor;
        let status = if complete { "✓" } else { "✗" };
        output.push_str(&format!(
            "{} {} (engine:{} sdk:{} tooling:{} editor:{})\n",
            status,
            domain.name,
            if domain.has_engine { "✓" } else { "✗" },
            if domain.has_sdk { "✓" } else { "✗" },
            if domain.has_tooling { "✓" } else { "✗" },
            if domain.has_editor { "✓" } else { "✗" },
        ));
    }
    output
}

fn format_vertical_slices(slices: &[VerticalSlice]) -> String {
    let mut output = String::new();
    for slice in slices {
        let complete = slice.has_engine && slice.has_sdk && slice.has_tooling && slice.has_editor;
        let status = if complete { "✓" } else { "✗" };
        output.push_str(&format!(
            "{} {} (engine:{} sdk:{} tooling:{} editor:{})\n",
            status,
            slice.name,
            if slice.has_engine { "✓" } else { "✗" },
            if slice.has_sdk { "✓" } else { "✗" },
            if slice.has_tooling { "✓" } else { "✗" },
            if slice.has_editor { "✓" } else { "✗" },
        ));
    }
    output
}
