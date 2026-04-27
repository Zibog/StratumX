//! Compatibility profile types.

use super::{Capability, CompatCapabilityId, CompatVersionId};
use crate::model::versions::CompatDomain;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CompatProfileId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CompatibilityProfile {
    ToolRuntime,
    EditorSurface,
    Automation,
    Diagnostics,
}

impl CompatibilityProfile {
    pub const ALL: &'static [Self] = &[
        Self::ToolRuntime,
        Self::EditorSurface,
        Self::Automation,
        Self::Diagnostics,
    ];
    pub fn required_capabilities(&self) -> &'static [Capability] {
        match self {
            Self::ToolRuntime => &[Capability::Snapshots, Capability::Controls],
            Self::EditorSurface => &[Capability::Snapshots, Capability::Observations],
            Self::Automation => &[Capability::Snapshots, Capability::Metrics],
            Self::Diagnostics => &[Capability::Observations, Capability::Metrics],
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::ToolRuntime => "tool_runtime",
            Self::EditorSurface => "editor_surface",
            Self::Automation => "automation",
            _ => "diagnostics",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsumerAudience {
    Editor,
    Tooling,
    Both,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerTableEntry {
    pub audience: ConsumerAudience,
    pub domain: CompatDomain,
    pub profile: CompatibilityProfile,
    pub cadence: CadenceHint,
    pub delivery_guarantee: DeliveryGuarantee,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CadenceHint {
    Once,
    OnChange,
    Periodic { ticks: u64 },
    Burst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryGuarantee {
    OrderedReliable,
    BestEffort,
    AtLeastOnce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileFact {
    pub profile_id: CompatProfileId,
    pub profile_name: String,
    pub allowed_version_set: Vec<CompatVersionId>,
    pub capability_set: Vec<Capability>,
    pub capability_id_set: Vec<CompatCapabilityId>,
    pub fallback_profile_id: Option<CompatProfileId>,
}

impl ProfileFact {
    pub fn new(
        profile_id: CompatProfileId,
        profile_name: &'static str,
        allowed_version_set: Vec<CompatVersionId>,
        capability_set: Vec<Capability>,
    ) -> Self {
        Self {
            profile_id,
            profile_name: profile_name.to_string(),
            allowed_version_set,
            capability_set,
            capability_id_set: Vec::new(),
            fallback_profile_id: None,
        }
    }
    pub fn with_capability_ids(mut self, ids: Vec<CompatCapabilityId>) -> Self {
        self.capability_id_set = ids;
        self
    }
    pub fn with_fallback(mut self, fallback: CompatProfileId) -> Self {
        self.fallback_profile_id = Some(fallback);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainProfileFact {
    pub domain: CompatDomain,
    pub applicable_profiles: Vec<CompatibilityProfile>,
    pub audience: ConsumerAudience,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileRegistry {
    pub profiles: Vec<ProfileFact>,
    pub domain_profiles: Vec<DomainProfileFact>,
    pub consumer_table: Vec<ConsumerTableEntry>,
}

impl ProfileRegistry {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            domain_profiles: Vec::new(),
            consumer_table: Vec::new(),
        }
    }
    pub fn with_profile(mut self, fact: ProfileFact) -> Self {
        self.profiles.push(fact);
        self
    }
    pub fn with_domain_profile(mut self, fact: DomainProfileFact) -> Self {
        self.domain_profiles.push(fact);
        self
    }
    pub fn with_consumer_table_entry(mut self, entry: ConsumerTableEntry) -> Self {
        self.consumer_table.push(entry);
        self
    }
    pub fn get_fallback(&self, profile: CompatibilityProfile) -> Option<CompatibilityProfile> {
        let fact = self.profiles.iter().find(|p| {
            matches!(
                (p.profile_name.as_str(), profile),
                ("tool_runtime", CompatibilityProfile::ToolRuntime)
                    | ("editor_surface", CompatibilityProfile::EditorSurface)
                    | ("automation", CompatibilityProfile::Automation)
                    | ("diagnostics", CompatibilityProfile::Diagnostics)
            )
        });
        fact.and_then(|f| {
            f.fallback_profile_id.map(|id| match id.0 {
                0 => CompatibilityProfile::ToolRuntime,
                1 => CompatibilityProfile::EditorSurface,
                2 => CompatibilityProfile::Automation,
                _ => CompatibilityProfile::Diagnostics,
            })
        })
    }
}

impl Default for ProfileRegistry {
    fn default() -> Self {
        Self::new()
    }
}
