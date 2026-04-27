use super::common::{string_args, suite_args, QualityContext, StageRecord};
use repo_hygiene::{HygieneChecker, HygieneRule, WaiverRegistry};
use std::collections::BTreeMap;

pub fn run(ctx: &QualityContext, verbose: bool) -> Result<(), String> {
    let mut stages = Vec::<StageRecord>::new();

    run_stage(
        ctx,
        &mut stages,
        "hygiene",
        "repo_hygiene::HygieneChecker",
        || {
            let waiver_path = ctx
                .repo_root
                .join("7.quality")
                .join("suites")
                .join("repo_hygiene")
                .join("waivers.toml");
            let waiver_registry = if waiver_path.exists() {
                WaiverRegistry::load_from_file(&waiver_path)
                    .map_err(|err| format!("failed to load waiver registry: {err}"))?
            } else {
                WaiverRegistry::default()
            };
            let checker = HygieneChecker::new(ctx.repo_root.clone(), waiver_registry);
            let report = checker.run_all_checks();
            if verbose {
                println!(
                    "hygiene: passed={}, failed={}, violations={}",
                    report.passed_checks,
                    report.failed_checks,
                    report.violations.len()
                );
                let mut by_rule = BTreeMap::<&'static str, usize>::new();
                for violation in &report.violations {
                    *by_rule.entry(rule_name(&violation.rule)).or_default() += 1;
                }
                for (rule, count) in by_rule {
                    println!("  - {}: {}", rule, count);
                }
                for violation in report.violations.iter().take(20) {
                    println!(
                        "    {}:{} [{}]",
                        violation.path.display(),
                        violation.line_number.unwrap_or(0),
                        rule_name(&violation.rule)
                    );
                }
            }
            if report.violations.is_empty() {
                Ok(())
            } else {
                Err(format!(
                    "hygiene checks found {} violations",
                    report.violations.len()
                ))
            }
        },
    )?;

    run_format_stage(ctx, &mut stages)?;
    run_cargo_stage(
        ctx,
        &mut stages,
        "tooling-build",
        &[
            "test",
            "-p",
            "stratumx_tooling_l6_1_command_envelopes",
            "-p",
            "stratumx_tooling_l6_0_tool_session",
            "--lib",
            "-j",
            "1",
        ],
    )?;
    run_cargo_stage(
        ctx,
        &mut stages,
        "spine-build",
        &[
            "check",
            "-p",
            "stratumx-editor-l7-0-editor-command-spine",
            "--lib",
            "-j",
            "1",
        ],
    )?;

    let sanity_args = suite_args(&[
        "editor_command_matrix",
        "editor_state_matrix",
        "editor_shell_matrix",
        "forbidden_shortcuts",
    ]);
    run_cargo_stage(ctx, &mut stages, "sanity", &string_args(&sanity_args))?;

    let smoke_args = suite_args(&["smoke"]);
    run_cargo_stage(ctx, &mut stages, "smoke", &string_args(&smoke_args))?;

    ctx.write_task_summary("verify", &stages, None)
}

fn rule_name(rule: &HygieneRule) -> &'static str {
    match rule {
        HygieneRule::LineLimit { .. } => "line_limit",
        HygieneRule::TestFileInSrc => "test_in_src",
        HygieneRule::TodoComment => "todo_comment",
        HygieneRule::AllowAttribute { .. } => "allow_attribute",
        HygieneRule::DocAssetInRuntime => "doc_asset_in_runtime",
        HygieneRule::TestAssetInRuntime => "test_asset_in_runtime",
        HygieneRule::TruthBypass => "truth_bypass",
        HygieneRule::DuplicateModule { .. } => "duplicate_module",
        HygieneRule::AppLogic => "app_logic",
        HygieneRule::StaticMut => "static_mut",
        HygieneRule::ExecuteBridge => "execute_bridge",
        HygieneRule::HostBypass => "host_bypass",
        HygieneRule::HardcodedPath => "hardcoded_path",
        HygieneRule::FakeTruth => "fake_truth",
    }
}

fn run_cargo_stage(
    ctx: &QualityContext,
    stages: &mut Vec<StageRecord>,
    stage: &str,
    args: &[&str],
) -> Result<(), String> {
    match ctx.run_cargo(stage, args) {
        Ok(record) => {
            stages.push(record);
            Ok(())
        }
        Err(err) => {
            let _ = ctx.write_task_summary("verify", stages, Some(&err));
            Err(err)
        }
    }
}

fn run_format_stage(ctx: &QualityContext, stages: &mut Vec<StageRecord>) -> Result<(), String> {
    let manifests = ctx.workspace_member_manifest_paths()?;
    match ctx.record_stage(
        "format",
        "cargo fmt --manifest-path <member>/Cargo.toml --check",
        || {
            for manifest in manifests {
                let manifest_str = manifest
                    .to_str()
                    .ok_or_else(|| format!("non-utf8 manifest path: {}", manifest.display()))?;
                ctx.run_cargo(
                    "format-member",
                    &["fmt", "--manifest-path", manifest_str, "--check"],
                )
                .map(|_| ())
                .map_err(|err| {
                    format!("format check failed for {}: {}", manifest.display(), err)
                })?;
            }
            Ok(())
        },
    ) {
        Ok(record) => {
            stages.push(record);
            Ok(())
        }
        Err(err) => {
            let _ = ctx.write_task_summary("verify", stages, Some(&err));
            Err(err)
        }
    }
}

fn run_stage<F>(
    ctx: &QualityContext,
    stages: &mut Vec<StageRecord>,
    name: &str,
    command: &str,
    f: F,
) -> Result<(), String>
where
    F: FnOnce() -> Result<(), String>,
{
    match ctx.record_stage(name, command, f) {
        Ok(record) => {
            stages.push(record);
            Ok(())
        }
        Err(err) => {
            let _ = ctx.write_task_summary("verify", stages, Some(&err));
            Err(err)
        }
    }
}
