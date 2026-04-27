pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

pub use stratumx_tooling_l6_0_tool_session::{
    BuildArtifact, ObjectHandle, PreviewResult, ReleasePackage, ToolingError, ToolingRuntime,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InspectorState {
    pub selected: Option<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BuildReleaseSurface {
    pub last_build: Option<BuildArtifact>,
    pub last_release: Option<ReleasePackage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EditorProduct {
    pub tooling: ToolingRuntime,
    pub build_release_surface: BuildReleaseSurface,
    pub inspector: InspectorState,
}
impl EditorProduct {
    pub fn refresh_from_tooling(&mut self) -> Result<(), ToolingError> {
        Ok(())
    }
    pub fn run_build(&mut self) -> Result<BuildArtifact, ToolingError> {
        let build = self.tooling.build_current();
        self.build_release_surface.last_build = Some(build.clone());
        self.refresh_from_tooling()?;
        Ok(build)
    }
    pub fn run_release(
        &mut self,
        channel: impl Into<String>,
    ) -> Result<ReleasePackage, ToolingError> {
        let build = self
            .build_release_surface
            .last_build
            .clone()
            .ok_or(ToolingError::NoBuildArtifact)?;
        let release = self.tooling.release_build(&build, channel.into(), true)?;
        self.build_release_surface.last_release = Some(release.clone());
        self.refresh_from_tooling()?;
        Ok(release)
    }
    pub fn preview_selected(&mut self) -> Result<Option<PreviewResult>, ToolingError> {
        let Some(handle) = self.inspector.selected else {
            return Ok(None);
        };
        let preview = self.tooling.preview_object(handle)?;
        Ok(Some(preview))
    }
}
