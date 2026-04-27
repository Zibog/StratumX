pub mod diagnostics_aggregator;
pub use diagnostics_aggregator::*;

#[cfg(feature = "desktop")]
pub mod desktop;

pub mod host;

pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

pub use stratumx_tooling_l6_0_tool_session::{
    ObjectClass, ObjectHandle, ToolObject, ToolSnapshot, ToolingError, ToolingRuntime,
    ValidationDiagnostic,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutlinerSystem {
    pub items: Vec<(ObjectHandle, String, bool)>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentBrowserSystem {
    pub visible_assets: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorField {
    pub key: String,
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorSystem {
    pub selected: Option<ObjectHandle>,
    pub label: Option<String>,
    pub fields: Vec<InspectorField>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildValidationReleaseSuite {
    pub validation_runs: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductionDashboardAndTraceability {
    pub counters: BTreeMap<String, usize>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsSurface {
    pub diagnostics: Vec<ValidationDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorProduct {
    pub tooling: ToolingRuntime,
    pub outliner: OutlinerSystem,
    pub content_browser: ContentBrowserSystem,
    pub inspector: InspectorSystem,
    pub diagnostics_surface: DiagnosticsSurface,
    pub build_suite: BuildValidationReleaseSuite,
    pub production_surface: ProductionDashboardAndTraceability,
}
impl EditorProduct {
    pub fn refresh_from_tooling(&mut self) -> Result<(), ToolingError> {
        let snapshot = self.tooling.snapshot();
        self.outliner.items = snapshot
            .objects
            .iter()
            .map(|object| (object.handle, object.label.clone(), object.active))
            .collect();
        self.content_browser.visible_assets = snapshot
            .objects
            .iter()
            .map(|object| format!("{:?}/{}", object.class, object.label))
            .collect();
        if let Some(selected) = self.inspector.selected {
            if let Some(object) = snapshot
                .objects
                .iter()
                .find(|object| object.handle == selected)
            {
                self.inspector.label = Some(object.label.clone());
                self.inspector.fields = object
                    .fields
                    .iter()
                    .map(|(key, value)| InspectorField {
                        key: key.clone(),
                        value: value.clone(),
                    })
                    .collect();
            }
        }
        self.diagnostics_surface.diagnostics = self.tooling.validate_snapshot();
        self.build_suite.validation_runs += 1;
        self.production_surface
            .counters
            .insert("objects".into(), snapshot.objects.len());
        self.production_surface.counters.insert(
            "diagnostics".into(),
            self.diagnostics_surface.diagnostics.len(),
        );
        Ok(())
    }
}
