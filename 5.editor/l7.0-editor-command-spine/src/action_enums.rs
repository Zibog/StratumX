//! Action Enums - Enumeration Types for Command Spine
//!
//! Contains all enumeration types used in the command spine architecture.

use serde::{Deserialize, Serialize};

// ============================================================================
// Disabled Reasons
// ============================================================================

/// Explicit reasons why an action cannot execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisabledReason {
    NoLegalProject,
    NoActiveWorld,
    NoSelection,
    NoLegalTargetSelected,
    MaterialRegistryUnavailable,
    AudioRegistryUnavailable,
    RouteUnsupportedByBuildProfile,
    DiagnosticsSourceUnavailable,
    CompareBaselineMissing,
    PlaceholderBlocksAction,
    ActiveTransactionForbidsMutation,
    NoLegalRecoveryAvailable,
}

impl DisabledReason {
    pub fn to_user_message(&self) -> &'static str {
        self.to_user_message_localized(&Locale::default())
    }
    pub fn to_tooltip(&self) -> &'static str {
        self.to_tooltip_localized(&Locale::default())
    }

    pub fn to_user_message_localized(&self, _locale: &Locale) -> &'static str {
        match self {
            Self::NoLegalProject => "No project is open",
            Self::NoActiveWorld => "No active world",
            Self::NoSelection => "No selection",
            Self::NoLegalTargetSelected => "No valid target selected",
            Self::MaterialRegistryUnavailable => "Material registry unavailable",
            Self::AudioRegistryUnavailable => "Audio registry unavailable",
            Self::RouteUnsupportedByBuildProfile => {
                "Operation not supported in current build profile"
            }
            Self::DiagnosticsSourceUnavailable => "Diagnostics system unavailable",
            Self::CompareBaselineMissing => "No baseline available for comparison",
            Self::PlaceholderBlocksAction => "Unresolved dependency blocks this action",
            Self::ActiveTransactionForbidsMutation => {
                "Cannot modify state during active transaction"
            }
            Self::NoLegalRecoveryAvailable => "No recovery action available",
        }
    }

    pub fn to_tooltip_localized(&self, _locale: &Locale) -> &'static str {
        match self {
            Self::NoLegalProject => {
                "This action requires an open project. Create or open a project first."
            }
            Self::NoActiveWorld => "This action requires an active world.",
            Self::NoSelection => "This action requires a selection.",
            Self::NoLegalTargetSelected => {
                "This action requires a valid selection. Select a target entity or object first."
            }
            Self::MaterialRegistryUnavailable => "Material registry is not available.",
            Self::AudioRegistryUnavailable => "Audio registry is not available.",
            Self::RouteUnsupportedByBuildProfile => {
                "The current build profile does not support this operation."
            }
            Self::DiagnosticsSourceUnavailable => "The diagnostics system is not available.",
            Self::CompareBaselineMissing => "This comparison requires a baseline reference.",
            Self::PlaceholderBlocksAction => "This action is blocked by an unresolved dependency.",
            Self::ActiveTransactionForbidsMutation => {
                "Cannot modify state while a transaction is active."
            }
            Self::NoLegalRecoveryAvailable => "No recovery action is available for this failure.",
        }
    }
}

// ============================================================================
// Denial Families
// ============================================================================

/// Categorized failure codes from tooling routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DenialFamily {
    MatProfileInvalid,
    MatBindingFailed,
    AudSourceInvalid,
    AudZoneConfigFailed,
    DstSimulationFailed,
    PrjNoWorkspace,
    PrjOpenFailed,
    PrjCloseFailed,
    WldOpenFailed,
    WldSaveFailed,
    WldCloseFailed,
    WldNoActive,
    TrnNoTerrain,
    TrnSculptFailed,
    TrnPaintFailed,
    EnvConfigFailed,
    RtKernelUnavailable,
    RtPlayFailed,
    RtPauseFailed,
    FlrInsufficient,
    FlrDegradeRungExhausted,
    RunInitFailed,
    RunExecFailed,
    DiagCaptureFailed,
    BldFailed,
    RelCertFailed,
    PnlOpenFailed,
    PnlCloseFailed,
}

impl DenialFamily {
    pub fn to_user_message(&self) -> &'static str {
        match self {
            Self::MatProfileInvalid => "Material profile is invalid",
            Self::MatBindingFailed => "Material binding failed",
            Self::AudSourceInvalid => "Audio source is invalid",
            Self::AudZoneConfigFailed => "Audio zone configuration failed",
            Self::DstSimulationFailed => "Terrain simulation failed",
            Self::PrjNoWorkspace => "No workspace available",
            Self::PrjOpenFailed => "Failed to open project",
            Self::PrjCloseFailed => "Failed to close project",
            Self::WldOpenFailed => "Failed to open world",
            Self::WldSaveFailed => "Failed to save world",
            Self::WldCloseFailed => "Failed to close world",
            Self::WldNoActive => "No active world",
            Self::TrnNoTerrain => "No terrain available",
            Self::TrnSculptFailed => "Terrain sculpting failed",
            Self::TrnPaintFailed => "Terrain painting failed",
            Self::EnvConfigFailed => "Environment configuration failed",
            Self::RtKernelUnavailable => "Runtime kernel unavailable",
            Self::RtPlayFailed => "Failed to start runtime",
            Self::RtPauseFailed => "Failed to pause runtime",
            Self::FlrInsufficient => "Hardware requirements not met",
            Self::FlrDegradeRungExhausted => "Quality degradation limit reached",
            Self::RunInitFailed => "Runtime initialization failed",
            Self::RunExecFailed => "Runtime execution failed",
            Self::DiagCaptureFailed => "Diagnostics capture failed",
            Self::BldFailed => "Build failed",
            Self::RelCertFailed => "Release certification failed",
            Self::PnlOpenFailed => "Failed to open panel",
            Self::PnlCloseFailed => "Failed to close panel",
        }
    }
}

// ============================================================================
// Locale
// ============================================================================

/// Locale identifier for localization support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Locale {
    #[default]
    English,
}

impl Locale {
    pub fn system() -> Self {
        Self::default()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disabled_reason_messages() {
        let reason = DisabledReason::NoLegalProject;
        assert_eq!(reason.to_user_message(), "No project is open");
    }

    #[test]
    fn test_denial_family_messages() {
        let denial = DenialFamily::WldOpenFailed;
        assert_eq!(denial.to_user_message(), "Failed to open world");
    }

    #[test]
    fn test_locale_default() {
        assert_eq!(Locale::default(), Locale::English);
    }
}
