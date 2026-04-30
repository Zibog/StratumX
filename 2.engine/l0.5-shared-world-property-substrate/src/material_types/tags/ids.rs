use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MaterialArchetypeId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MaterialStackId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MaterialId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ResponseProfileId(pub u16);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SurfaceFamilyId(pub String);

impl SurfaceFamilyId {
    pub const MATERIAL_PREFIX: &'static str = "surface.material.";

    pub fn from_material_id(material_id: MaterialId) -> Self {
        let mut id = String::from(Self::MATERIAL_PREFIX);
        id.push_str(&material_id.0.to_string());
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ResponseFamilyId(pub String);

impl ResponseFamilyId {
    pub const PHYSICAL_THERMAL_PREFIX: &'static str = "matresp.physical.thermal.";

    pub fn physical_thermal(suffix: &'static str) -> Self {
        let mut id = String::from(Self::PHYSICAL_THERMAL_PREFIX);
        id.push_str(suffix);
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AftermathFamilyId(pub String);

impl AftermathFamilyId {
    pub const REMAIN_INTACT: &'static str = "burn.aftermath.remain_intact";
    pub const CONVERT_TO_ASH: &'static str = "burn.aftermath.convert_to_ash";
    pub const CONVERT_TO_CHARRED: &'static str = "burn.aftermath.convert_to_charred";
    pub const DISINTEGRATE: &'static str = "burn.aftermath.disintegrate";

    pub fn remain_intact() -> Self {
        Self(Self::REMAIN_INTACT.to_owned())
    }

    pub fn convert_to_ash() -> Self {
        Self(Self::CONVERT_TO_ASH.to_owned())
    }

    pub fn convert_to_charred() -> Self {
        Self(Self::CONVERT_TO_CHARRED.to_owned())
    }

    pub fn disintegrate() -> Self {
        Self(Self::DISINTEGRATE.to_owned())
    }

    pub fn requires_persistence(&self) -> bool {
        self.as_str() != Self::REMAIN_INTACT
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

macro_rules! impl_family_id_string_bridge {
    ($ty:ty) => {
        impl From<String> for $ty {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $ty {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        impl From<$ty> for String {
            fn from(value: $ty) -> Self {
                value.0
            }
        }

        impl AsRef<str> for $ty {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Borrow<str> for $ty {
            fn borrow(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

impl_family_id_string_bridge!(SurfaceFamilyId);
impl_family_id_string_bridge!(ResponseFamilyId);
impl_family_id_string_bridge!(AftermathFamilyId);
