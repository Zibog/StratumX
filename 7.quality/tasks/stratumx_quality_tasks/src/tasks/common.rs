use serde_json::json;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct QualityContext {
    pub repo_root: PathBuf,
    pub generated_root: PathBuf,
}

#[derive(Debug, Clone)]
pub struct StageRecord {
    pub name: String,
    pub command: String,
    pub duration_ms: u128,
}

impl QualityContext {
    pub fn discover() -> Self {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .expect("workspace root");
        let generated_root = repo_root.join("7.quality").join("data").join("generated");
        let ctx = Self {
            repo_root,
            generated_root,
        };
        ctx.ensure_layout();
        ctx
    }

    pub fn ensure_layout(&self) {
        for relative in ["bench", "evidence", "metrics", "smoke", "test-results"] {
            let _ = fs::create_dir_all(self.generated_root.join(relative));
        }
    }

    pub fn run_cargo(&self, stage: &str, args: &[&str]) -> Result<StageRecord, String> {
        let command = format!("cargo {}", args.join(" "));
        let start = Instant::now();
        let status = Command::new("cargo")
            .current_dir(&self.repo_root)
            .args(args)
            .status()
            .map_err(|err| format!("failed to launch {command}: {err}"))?;
        let duration_ms = start.elapsed().as_millis();
        if !status.success() {
            return Err(format!("{command} failed with status {status}"));
        }

        Ok(StageRecord {
            name: stage.to_string(),
            command,
            duration_ms,
        })
    }

    pub fn cargo_output(&self, args: &[&str]) -> Result<String, String> {
        let output = Command::new("cargo")
            .current_dir(&self.repo_root)
            .args(args)
            .output()
            .map_err(|err| format!("failed to launch cargo {:?}: {err}", args))?;
        if !output.status.success() {
            return Err(format!(
                "cargo {:?} failed with status {}",
                args, output.status
            ));
        }
        String::from_utf8(output.stdout).map_err(|err| err.to_string())
    }

    pub fn workspace_member_manifest_paths(&self) -> Result<Vec<PathBuf>, String> {
        let metadata = self.cargo_output(&["metadata", "--format-version", "1", "--no-deps"])?;
        let value: serde_json::Value =
            serde_json::from_str(&metadata).map_err(|err| err.to_string())?;

        let workspace_members = value
            .get("workspace_members")
            .and_then(|members| members.as_array())
            .ok_or_else(|| "cargo metadata missing workspace_members".to_string())?
            .iter()
            .filter_map(|member| member.as_str().map(str::to_owned))
            .collect::<HashSet<_>>();

        let mut manifests = value
            .get("packages")
            .and_then(|packages| packages.as_array())
            .ok_or_else(|| "cargo metadata missing packages".to_string())?
            .iter()
            .filter(|package| {
                package
                    .get("id")
                    .and_then(|id| id.as_str())
                    .map(|id| workspace_members.contains(id))
                    .unwrap_or(false)
            })
            .filter_map(|package| {
                package
                    .get("manifest_path")
                    .and_then(|path| path.as_str())
                    .map(PathBuf::from)
            })
            .collect::<Vec<_>>();

        manifests.sort();
        Ok(manifests)
    }

    pub fn record_stage<F>(&self, name: &str, command: &str, f: F) -> Result<StageRecord, String>
    where
        F: FnOnce() -> Result<(), String>,
    {
        let start = Instant::now();
        f()?;
        Ok(StageRecord {
            name: name.to_string(),
            command: command.to_string(),
            duration_ms: start.elapsed().as_millis(),
        })
    }

    pub fn write_text(&self, relative: &str, content: &str) -> Result<(), String> {
        let path = self.generated_root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| err.to_string())?;
        }
        fs::write(path, content).map_err(|err| err.to_string())
    }

    pub fn write_task_summary(
        &self,
        command_name: &str,
        stages: &[StageRecord],
        failure: Option<&str>,
    ) -> Result<(), String> {
        let status = if failure.is_some() {
            "failed"
        } else {
            "passed"
        };
        let total_duration_ms: u128 = stages.iter().map(|stage| stage.duration_ms).sum();
        let markdown = render_markdown_summary(command_name, status, stages, failure);
        self.write_text(
            &format!("test-results/{}-summary.md", command_name),
            &markdown,
        )?;
        let json = json!({
            "command": command_name,
            "status": status,
            "total_duration_ms": total_duration_ms,
            "stages": stages.iter().map(|stage| json!({
                "name": stage.name,
                "command": stage.command,
                "duration_ms": stage.duration_ms,
            })).collect::<Vec<_>>(),
            "failure": failure,
        });
        self.write_text(
            &format!("test-results/{}-summary.json", command_name),
            &serde_json::to_string_pretty(&json).map_err(|err| err.to_string())?,
        )
    }
}

pub fn suite_args(packages: &[&str]) -> Vec<String> {
    let mut args = vec!["test".to_string()];
    for package in packages {
        args.push("-p".to_string());
        args.push((*package).to_string());
    }
    args.extend(["--lib".to_string(), "-j".to_string(), "1".to_string()]);
    args
}

pub fn string_args(args: &[String]) -> Vec<&str> {
    args.iter().map(|arg| arg.as_str()).collect()
}

pub fn read_summary_duration(root: &Path) -> u128 {
    let mut total = 0;
    let dir = root
        .join("7.quality")
        .join("data")
        .join("generated")
        .join("test-results");
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return 0,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let Ok(content) = fs::read_to_string(path) else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) else {
            continue;
        };
        total += value
            .get("total_duration_ms")
            .and_then(|value| value.as_u64())
            .unwrap_or_default() as u128;
    }
    total
}

fn render_markdown_summary(
    command_name: &str,
    status: &str,
    stages: &[StageRecord],
    failure: Option<&str>,
) -> String {
    let mut body = format!("# {}\n\nStatus: `{}`\n\n", command_name, status);
    body.push_str("| Stage | Command | Duration (ms) |\n");
    body.push_str("| --- | --- | ---: |\n");
    for stage in stages {
        body.push_str(&format!(
            "| {} | `{}` | {} |\n",
            stage.name, stage.command, stage.duration_ms
        ));
    }
    if let Some(failure) = failure {
        body.push_str(&format!("\nFailure: {}\n", failure));
    }
    body
}
