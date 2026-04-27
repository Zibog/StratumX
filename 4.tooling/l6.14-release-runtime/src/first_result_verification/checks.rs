// First result verification checks
//
// Implements the "ты победил" signature verification chain

use serde::{Deserialize, Serialize};

pub const FIRST_RESULT_SIGNATURE: &str = "ты победил";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirstResultVerification {
    pub verification_id: String,
    pub project_id: String,
    pub build_id: String,
    pub launch_timestamp: u64,
    pub runtime_signature: String,
    pub verified: bool,
    pub diagnostics_trace: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FirstResultVerifier {
    verifications: Vec<FirstResultVerification>,
}

impl Default for FirstResultVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl FirstResultVerifier {
    pub fn new() -> Self {
        Self {
            verifications: Vec::new(),
        }
    }

    pub fn verify_signature(
        &mut self,
        verification_id: String,
        project_id: String,
        build_id: String,
        runtime_signature: String,
        diagnostics_trace: Vec<String>,
    ) -> FirstResultVerification {
        let verified = runtime_signature == FIRST_RESULT_SIGNATURE;

        let verification = FirstResultVerification {
            verification_id: verification_id.clone(),
            project_id,
            build_id,
            launch_timestamp: 0, // Would use real timestamp
            runtime_signature,
            verified,
            diagnostics_trace,
        };

        self.verifications.push(verification.clone());
        verification
    }

    pub fn get_verification(&self, verification_id: &str) -> Option<&FirstResultVerification> {
        self.verifications
            .iter()
            .find(|v| v.verification_id == verification_id)
    }

    pub fn get_last_verification(&self) -> Option<&FirstResultVerification> {
        self.verifications.last()
    }

    pub fn is_verified(&self, verification_id: &str) -> bool {
        self.get_verification(verification_id)
            .map(|v| v.verified)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_signature_success() {
        let mut verifier = FirstResultVerifier::new();
        let result = verifier.verify_signature(
            "v1".to_string(),
            "proj1".to_string(),
            "build1".to_string(),
            FIRST_RESULT_SIGNATURE.to_string(),
            vec!["trace1".to_string()],
        );
        assert!(result.verified);
    }

    #[test]
    fn test_verify_signature_failure() {
        let mut verifier = FirstResultVerifier::new();
        let result = verifier.verify_signature(
            "v1".to_string(),
            "proj1".to_string(),
            "build1".to_string(),
            "wrong_signature".to_string(),
            vec![],
        );
        assert!(!result.verified);
    }

    #[test]
    fn test_is_verified() {
        let mut verifier = FirstResultVerifier::new();
        verifier.verify_signature(
            "v1".to_string(),
            "proj1".to_string(),
            "build1".to_string(),
            FIRST_RESULT_SIGNATURE.to_string(),
            vec![],
        );
        assert!(verifier.is_verified("v1"));
        assert!(!verifier.is_verified("v2"));
    }
}
