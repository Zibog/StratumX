// Build/Export/Launch chain evidence types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildArtifact {
    pub build_id: String,
    pub project_id: String,
    pub target_platform: String,
    pub executable_path: String,
    pub build_timestamp: u64,
    pub build_profile: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportArtifact {
    pub export_id: String,
    pub build_id: String,
    pub export_path: String,
    pub export_timestamp: u64,
    pub launchable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchTrace {
    pub launch_id: String,
    pub export_id: String,
    pub launch_timestamp: u64,
    pub process_id: Option<u32>,
    pub exit_code: Option<i32>,
    pub runtime_diagnostics: Vec<String>,
    pub runtime_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReleaseChain {
    builds: Vec<BuildArtifact>,
    exports: Vec<ExportArtifact>,
    launches: Vec<LaunchTrace>,
}

impl Default for ReleaseChain {
    fn default() -> Self {
        Self::new()
    }
}

impl ReleaseChain {
    pub fn new() -> Self {
        Self {
            builds: Vec::new(),
            exports: Vec::new(),
            launches: Vec::new(),
        }
    }

    pub fn register_build(&mut self, build: BuildArtifact) {
        self.builds.push(build);
    }

    pub fn register_export(&mut self, export: ExportArtifact) {
        self.exports.push(export);
    }

    pub fn register_launch(&mut self, launch: LaunchTrace) {
        self.launches.push(launch);
    }

    pub fn get_build(&self, build_id: &str) -> Option<&BuildArtifact> {
        self.builds.iter().find(|b| b.build_id == build_id)
    }

    pub fn get_export(&self, export_id: &str) -> Option<&ExportArtifact> {
        self.exports.iter().find(|e| e.export_id == export_id)
    }

    pub fn get_launch(&self, launch_id: &str) -> Option<&LaunchTrace> {
        self.launches.iter().find(|l| l.launch_id == launch_id)
    }

    pub fn get_last_build(&self) -> Option<&BuildArtifact> {
        self.builds.last()
    }

    pub fn get_last_export(&self) -> Option<&ExportArtifact> {
        self.exports.last()
    }

    pub fn get_last_launch(&self) -> Option<&LaunchTrace> {
        self.launches.last()
    }

    pub fn verify_chain(&self, launch_id: &str) -> Result<ChainVerification, String> {
        let launch = self
            .get_launch(launch_id)
            .ok_or_else(|| format!("Launch {} not found", launch_id))?;

        let export = self
            .get_export(&launch.export_id)
            .ok_or_else(|| format!("Export {} not found", launch.export_id))?;

        let build = self
            .get_build(&export.build_id)
            .ok_or_else(|| format!("Build {} not found", export.build_id))?;

        Ok(ChainVerification {
            launch_id: launch_id.to_string(),
            export_id: export.export_id.clone(),
            build_id: build.build_id.clone(),
            project_id: build.project_id.clone(),
            chain_valid: true,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChainVerification {
    pub launch_id: String,
    pub export_id: String,
    pub build_id: String,
    pub project_id: String,
    pub chain_valid: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_release_chain_registration() {
        let mut chain = ReleaseChain::new();

        let build = BuildArtifact {
            build_id: "b1".to_string(),
            project_id: "p1".to_string(),
            target_platform: "windows".to_string(),
            executable_path: "/path".to_string(),
            build_timestamp: 0,
            build_profile: "release".to_string(),
        };
        chain.register_build(build);

        let export = ExportArtifact {
            export_id: "e1".to_string(),
            build_id: "b1".to_string(),
            export_path: "/export".to_string(),
            export_timestamp: 0,
            launchable: true,
        };
        chain.register_export(export);

        let launch = LaunchTrace {
            launch_id: "l1".to_string(),
            export_id: "e1".to_string(),
            launch_timestamp: 0,
            process_id: Some(1234),
            exit_code: Some(0),
            runtime_diagnostics: vec![],
            runtime_signature: Some("test".to_string()),
        };
        chain.register_launch(launch);

        assert!(chain.get_build("b1").is_some());
        assert!(chain.get_export("e1").is_some());
        assert!(chain.get_launch("l1").is_some());
    }

    #[test]
    fn test_verify_chain() {
        let mut chain = ReleaseChain::new();

        chain.register_build(BuildArtifact {
            build_id: "b1".to_string(),
            project_id: "p1".to_string(),
            target_platform: "windows".to_string(),
            executable_path: "/path".to_string(),
            build_timestamp: 0,
            build_profile: "release".to_string(),
        });
        chain.register_export(ExportArtifact {
            export_id: "e1".to_string(),
            build_id: "b1".to_string(),
            export_path: "/export".to_string(),
            export_timestamp: 0,
            launchable: true,
        });
        chain.register_launch(LaunchTrace {
            launch_id: "l1".to_string(),
            export_id: "e1".to_string(),
            launch_timestamp: 0,
            process_id: None,
            exit_code: None,
            runtime_diagnostics: vec![],
            runtime_signature: None,
        });

        let verification = chain.verify_chain("l1").unwrap();
        assert!(verification.chain_valid);
        assert_eq!(verification.project_id, "p1");
    }
}
