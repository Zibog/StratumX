use super::enums::PropertyDomain;
use super::ids::{MaterialId, ResponseProfileId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialConfig {
    pub fallback_descriptor: MaterialDescriptor,
    pub default_reaction: ReactionRow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialDescriptor {
    pub material_id: MaterialId,
    pub label: String,
    pub property_domains: Vec<PropertyDomain>,
    pub response_profile: ResponseProfileId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactionRow {
    pub response_profile: ResponseProfileId,
    pub coefficients: [u16; 4],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialLookupResult {
    pub descriptor: MaterialDescriptor,
    pub reaction: ReactionRow,
    pub used_fallback: bool,
}
