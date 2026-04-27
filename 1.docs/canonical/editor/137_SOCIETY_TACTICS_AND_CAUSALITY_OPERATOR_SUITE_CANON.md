# Society Tactics And Causality Operator Suite Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Merge society, tactics, and causality explanation into one operator suite so the world remains inspectable at human scale.

## Suite tabs
- needs / economy / reputation
- crime / punishment / base formation
- squad intent / suppression / flank reservation
- cover invalidation and retreat / regroup
- why-this-happened explanation rail

## Required inspector fields
- `need_vector_ref`
- `status_delta_ref`
- `crime_event_ref`
- `shared_intent_code`
- `cover_validity_digest_ref`
- `cause_chain_refs`

## Required actions
- inspect first crime turn;
- inspect who witnessed or remembered the act;
- inspect squad shared intent and cover invalidation;
- inspect first blocker in a causal chain;
- hand off capture to compare and certification surfaces.

## Disabled reasons
`SOC_DISABLED_NO_AGENT_OR_SQUAD_SCOPE`, `SOC_DISABLED_CAUSE_CHAIN_MISSING`, `SOC_DISABLED_REPUTATION_LEDGER_MISSING`
