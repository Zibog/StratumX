//! Compatibility verdict helper functions
//!
//! Provides helper functions for negotiation and downgrade paths.

use super::super::versions::BridgeVersion;

/// Negotiation helper: produces a downgrade path from a presented version
/// to the closest supported version.
pub fn negotiate_downgrade(
    presented: BridgeVersion,
    supported_versions: &[BridgeVersion],
) -> Option<BridgeVersion> {
    // Find the highest supported version that is <= presented
    supported_versions
        .iter()
        .filter(|v| v.major == presented.major && v.minor <= presented.minor)
        .max()
        .copied()
}
