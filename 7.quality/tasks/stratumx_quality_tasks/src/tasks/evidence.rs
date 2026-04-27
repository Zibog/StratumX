use super::common::QualityContext;
use std::fs;
use stratumx_repo_hygiene_support::{quality_suite_names, scan_inline_test_debt};
use stratumx_tooling_l6_1_command_envelopes::canonical_route_manifest;

pub fn run(ctx: &QualityContext) -> Result<(), String> {
    let suites = quality_suite_names(&ctx.repo_root);
    let route_manifest = canonical_route_manifest();
    let inline_test_debt = scan_inline_test_debt(&ctx.repo_root);
    let artifacts = generated_artifacts(&ctx.generated_root);

    let mut markdown = String::from("# evidence pack\n\n");
    markdown.push_str(&format!(
        "- canonical_route_rows: {}\n- suite_count: {}\n- inline_test_debt: {}\n",
        route_manifest.route_count,
        suites.len(),
        inline_test_debt.len()
    ));
    markdown.push_str("\n## Artifacts\n");
    for artifact in artifacts {
        markdown.push_str(&format!("- {}\n", artifact));
    }

    ctx.write_text("evidence/evidence-pack.md", &markdown)?;
    ctx.write_task_summary("evidence", &[], None)
}

fn generated_artifacts(root: &std::path::Path) -> Vec<String> {
    let mut artifacts = Vec::new();
    walk_files(root, &mut |path| {
        let display = path
            .strip_prefix(root)
            .unwrap_or(path)
            .display()
            .to_string();
        artifacts.push(display);
    });
    artifacts.sort();
    artifacts
}

fn walk_files(dir: &std::path::Path, visit: &mut dyn FnMut(&std::path::Path)) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_files(&path, visit);
        } else {
            visit(&path);
        }
    }
}
