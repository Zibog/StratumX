# World Causality Explanation Object Model And Trace Families Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own explainability at world scale so operators can ask “why did this happen?” without folklore.

## Explanation object model

| Field | Meaning |
|---|---|
| `subject_ref` | object, actor, squad, field, or world phenomenon being explained |
| `phenomenon_family` | fire, leak, migration, crime turn, weather arrival, cover loss, and so on |
| `cause_chain_refs` | ordered upstream causes |
| `blocking_or_enabling_conditions` | conditions that allowed or prevented the outcome |
| `confidence_posture` | exact, reduced, inferred, or incomplete |
| `retention_horizon` | how long the explanation remains queryable |
| `evidence_refs` | packets, captures, or bundles that support the answer |

## Mandatory trace families
- fire spread and suppression;
- leak stop, overflow, and evaporation;
- squad flank, suppression, retreat, and cover invalidation;
- camp or base formation;
- migration wave arrival;
- weather-front arrival;
- crime turn and reputation shift;
- photoreal downgrade and first blocked feature.

## Retention law
Explanations may be reduced or summarized over time, but a retained explanation object must never silently lose its first divergence or first enabling condition.

## Failure families
- `causality.subject_missing`
- `causality.chain_incomplete`
- `causality.retention_gap`
- `causality.editor_route_unavailable`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
