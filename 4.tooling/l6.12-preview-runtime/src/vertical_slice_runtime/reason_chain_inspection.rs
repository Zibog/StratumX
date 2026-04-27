// Vertical Slice Runtime - Reason Chain Inspection

use link_egress_observations::ReasonChainEntryDto;

impl super::startup::VerticalSliceSession {
    pub fn authoring_inspect_reason_chain_npc(
        &self,
        npc_id: u32,
    ) -> Result<Vec<ReasonChainEntryDto>, String> {
        // Return a deterministic inspection row until runtime reason-chain querying is exposed here.
        Ok(vec![ReasonChainEntryDto {
            trace_id: format!("npc_{}_trace_1", npc_id),
            timestamp: 0.0,
            subject_id: Some(npc_id),
            category: "NeedEscalation".to_string(),
            summary: format!("NPC {} hunger escalated", npc_id),
        }])
    }

    pub fn authoring_inspect_reason_chain_scope(&self) -> Result<Vec<ReasonChainEntryDto>, String> {
        // Return a deterministic scope row until runtime reason-chain querying is exposed here.
        Ok(vec![ReasonChainEntryDto {
            trace_id: "scope_trace_1".to_string(),
            timestamp: 0.0,
            subject_id: None,
            category: "SystemEvent".to_string(),
            summary: "Simulation started".to_string(),
        }])
    }

    pub fn authoring_get_reason_trace_stats(&self) -> Result<usize, String> {
        // Report zero until runtime reason-chain statistics are surfaced through this session seam.
        Ok(0)
    }
}
