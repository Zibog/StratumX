# Project Bootstrap, Save, Build, and Export Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
    This catalog freezes the typed bridge for this surface family.
    The sdk is not allowed to be vague here.
    Every packet and observation named below is part of the active bridge.

    ## Packet and observation families
    - ProjectIdentityPacket
- ProjectSavePacket
- BuildRequestPacket
- ExportRequestPacket

    ## Field-level schema policy
    Every family in this catalog must define:
    - stable field names;
    - required vs optional fields;
    - enum tables;
    - legality and failure codes;
    - a `schema_revision`;
    - normalization rules for observations;
    - compatibility rules for additive and breaking changes.

    ## Core envelope fields
    | Field | Meaning |
|---|---|
| 58_status | ok / degraded / denied / open |
| schema_revision | integer >= 1 |
| scope | project / technology / package / global |
| tier | operator / diagnostics / evidence / certification |


    ## Legality and failure law
    Each result family must expose:
    - one verdict code;
    - one failure class when denied or degraded;
    - one source scope identifying which package emitted the result; and
    - one timestamp or monotonic sequence suitable for compare/replay.

    ## Current posture
    `doc_closed_impl_open` unless a narrower path is promoted in the implementation ledger.
