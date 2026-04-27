// FIRST RESULT VERIFICATION SYSTEM
// Implements the "ты победил" signature verification chain

mod checks;
mod evidence;

pub use checks::{FirstResultVerification, FirstResultVerifier, FIRST_RESULT_SIGNATURE};
pub use evidence::{BuildArtifact, ChainVerification, ExportArtifact, LaunchTrace, ReleaseChain};
