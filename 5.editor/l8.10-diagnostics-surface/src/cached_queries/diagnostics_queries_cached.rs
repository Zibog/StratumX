use crate::DiagnosticMessage;

use super::CachedStateQueries;

impl<'a> CachedStateQueries<'a> {
    pub(crate) fn diagnostics_slice(&self) -> &[DiagnosticMessage] {
        &self.diagnostics_state.messages
    }

    pub(crate) fn diagnostic_count(&self) -> usize {
        self.diagnostics_state.messages.len()
    }
}
