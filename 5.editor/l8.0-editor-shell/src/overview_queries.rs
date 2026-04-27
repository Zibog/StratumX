use crate::editor_product_model::EditorProduct;
use crate::suite_overview::SuiteOverview;
use std::sync::Arc;
use stratumx_tooling_l6_0_tool_session::{ObjectClass, ToolObject, ToolSnapshot};

impl EditorProduct {
    pub fn snapshot(&self) -> Arc<ToolSnapshot> {
        self.tooling.snapshot()
    }

    pub fn suite_overview(&self) -> SuiteOverview {
        let snapshot = self.tooling.snapshot();
        SuiteOverview {
            worlds: snapshot
                .objects
                .iter()
                .filter(|o| o.class == ObjectClass::World)
                .count(),
            scenes: snapshot
                .objects
                .iter()
                .filter(|o| o.class == ObjectClass::Scene)
                .count(),
            terrains: snapshot
                .objects
                .iter()
                .filter(|o| o.class == ObjectClass::Terrain)
                .count(),
            materials: snapshot
                .objects
                .iter()
                .filter(|o| o.class == ObjectClass::Material)
                .count(),
            logic_nodes: snapshot
                .objects
                .iter()
                .filter(|o| o.class == ObjectClass::Logic)
                .count(),
            assets: snapshot
                .objects
                .iter()
                .filter(|o| o.class == ObjectClass::Asset)
                .count(),
        }
    }

    pub fn selected_object(&self) -> Option<ToolObject> {
        let handle = self.inspector.selected?;
        self.tooling
            .snapshot()
            .objects
            .iter()
            .find(|object| object.handle == handle)
            .cloned()
    }

    pub fn transaction_summaries(&self) -> Vec<String> {
        self.tooling
            .ledger()
            .iter()
            .rev()
            .take(128)
            .map(|tx| {
                format!(
                    "#{} {:?}/{:?}: {}",
                    tx.order, tx.origin, tx.budget, tx.summary
                )
            })
            .collect()
    }
}
