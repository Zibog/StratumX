// Build and Release Operations

use super::runtime::ToolingRuntime;
use super::types::*;

impl ToolingRuntime {
    pub fn build_current(&mut self) -> BuildArtifact {
        let artifact = BuildArtifact {
            digest: format!("build:{}:{}", self.objects.len(), self.transactions.len()),
            object_count: self.objects.len(),
            manifest: BuildManifest {
                digest: format!("build:{}:{}", self.objects.len(), self.transactions.len()),
            },
        };
        self.last_build = Some(artifact.clone());
        artifact
    }

    pub fn release_build(
        &mut self,
        build: &BuildArtifact,
        channel: impl Into<String>,
        signed: bool,
    ) -> Result<ReleasePackage, ToolingError> {
        let channel = channel.into();
        let release = ReleasePackage {
            build_digest: build.digest.clone(),
            channel: channel.clone(),
            signed,
            manifest: ReleaseManifest {
                digest: format!("release:{}:{}:{}", build.digest, channel, signed as u8),
            },
        };
        self.releases.push(release.clone());
        Ok(release)
    }
}
