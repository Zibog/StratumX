# LLM Dialogue Boundary Memory Audit And Fallback Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the safety, memory, audit, fallback, and semantic-boundary law for LLM-driven dialogue.

## Exact truth objects

| Object | Role |
|---|---|
| `DialogueSemanticState` | authoritative meaning and allowed semantic deltas |
| `DialoguePromptContract` | prompt template family, allowed slots, and forbidden instructions |
| `DialogueMemoryWindow` | bounded memory slice used for one generation request |
| `DialogueAuditRecord` | request, version, and fallback lineage for every generation |
| `DialogueFallbackVerdict` | offline or degraded response class |
| `DialogueLatencyBudget` | request budget and escalation posture |

## Hard boundary law
- generated dialogue may not directly mutate canonical world truth;
- semantic intent and consequence are authoritative, prose is derived;
- every generation request retains prompt contract id, memory window id, and audit record id;
- offline or fallback mode must remain lawful and attributable;
- save/load stores semantic state and audit lineage, not opaque prose blobs as truth.

## Mandatory publications
- `packet.living.dialogue_request.v1`
- `packet.living.dialogue_result.v1`
- `packet.living.dialogue_fallback.v1`
- explanation anchors consumed by editor `137`

## Failure families
- `dialogue.prompt_contract_missing`
- `dialogue.memory_window_illegal`
- `dialogue.truth_mutation_forbidden`
- `dialogue.fallback_unavailable`
- `dialogue.audit_record_missing`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
