use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RuntimeCheapnessRung {
    #[default]
    Unset,
    HotExact,
    LocalReduced,
    CachedLocal,
    StatisticalFar,
    RetainedAftermathOnly,
}

impl RuntimeCheapnessRung {
    pub fn is_configured(self) -> bool {
        !matches!(self, RuntimeCheapnessRung::Unset)
    }
}
