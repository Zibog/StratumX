use super::common::{read_summary_duration, QualityContext};
use serde_json::json;
use std::fs;
use stratumx_repo_hygiene_support::{
    declared_test_count, quality_suite_names, scan_inline_test_debt,
};
use stratumx_tooling_l6_1_command_envelopes::canonical_route_manifest;

pub fn run(ctx: &QualityContext) -> Result<(), String> {
    let metadata = ctx
        .cargo_output(&["metadata", "--format-version", "1", "--no-deps"])
        .ok();
    let package_count = metadata
        .as_deref()
        .and_then(|output| serde_json::from_str::<serde_json::Value>(output).ok())
        .and_then(|value| {
            value
                .get("packages")
                .and_then(|packages| packages.as_array())
                .map(|packages| packages.len())
        })
        .unwrap_or_default();

    let suite_names = quality_suite_names(&ctx.repo_root);
    let inline_test_debt = scan_inline_test_debt(&ctx.repo_root);
    let route_manifest = canonical_route_manifest();
    let largest_source_file = largest_source_file(&ctx.repo_root);
    let total_duration_ms = read_summary_duration(&ctx.repo_root);

    let markdown = format!(
        "# metrics\n\n- workspace_packages: {}\n- quality_suites: {}\n- declared_tests: {}\n- route_coverage: {}\n- inline_test_debt: {}\n- accumulated_suite_duration_ms: {}\n- largest_source_file: {}\n",
        package_count,
        suite_names.len(),
        declared_test_count(&ctx.repo_root),
        route_manifest.route_count,
        inline_test_debt.len(),
        total_duration_ms,
        largest_source_file,
    );
    ctx.write_text("metrics/metrics-summary.md", &markdown)?;

    let json = json!({
        "workspace_packages": package_count,
        "quality_suites": suite_names,
        "declared_tests": declared_test_count(&ctx.repo_root),
        "route_coverage": route_manifest.route_count,
        "inline_test_debt": inline_test_debt.len(),
        "accumulated_suite_duration_ms": total_duration_ms,
        "largest_source_file": largest_source_file,
    });
    ctx.write_text(
        "metrics/metrics-summary.json",
        &serde_json::to_string_pretty(&json).map_err(|err| err.to_string())?,
    )?;
    ctx.write_task_summary("metrics", &[], None)
}

fn largest_source_file(repo_root: &std::path::Path) -> String {
    let mut largest: Option<(usize, String)> = None;
    for root in ["2.engine", "3.sdk", "4.tooling", "5.editor", "6.apps"] {
        walk_rs_files(&repo_root.join(root), &mut |path| {
            if let Ok(content) = fs::read_to_string(path) {
                let count = content.lines().count();
                let display = path
                    .strip_prefix(repo_root)
                    .unwrap_or(path)
                    .display()
                    .to_string();
                if largest
                    .as_ref()
                    .map(|(lines, _)| count > *lines)
                    .unwrap_or(true)
                {
                    largest = Some((count, display));
                }
            }
        });
    }
    largest
        .map(|(lines, path)| format!("{} ({} lines)", path, lines))
        .unwrap_or_else(|| "unavailable".to_string())
}

fn walk_rs_files(dir: &std::path::Path, visit: &mut dyn FnMut(&std::path::Path)) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                if name == "target" || name.starts_with('.') {
                    continue;
                }
            }
            walk_rs_files(&path, visit);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            visit(&path);
        }
    }
}
