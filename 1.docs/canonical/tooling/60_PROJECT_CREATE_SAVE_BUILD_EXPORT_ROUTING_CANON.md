# Project Create, Save, Build, and Export Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
    This routing contract freezes the tooling execution layer for project create/save/build/export.
    Tooling is the owner of transaction state, retries, artifact routing, cache ownership, invalidation, and diagnostics publication.

    ## Required intent schema
    - `intent_id`
    - `operator_session_id`
    - `target_ref`
    - `requested_mode`
    - `preview_or_commit`
    - `evidence_capture_requested`
    - `expected_result_kind`

    ## Transaction state machine
    | State | Meaning |
|---|---|
| received | intent accepted into tooling |
| validated | preconditions checked |
| prepared | inputs normalized and ownership resolved |
| executing | route is actively executing |
| publishing | results are being published upward |
| retryable_failure | failure may be retried within budget |
| rolled_back | state or artifact rollback completed |
| completed | results published and focus rules emitted |


    ## Ownership and artifact law
    Tooling owns:
    - transaction identity;
    - retry budget;
    - rollback decision;
    - artifact retention and reveal semantics;
    - cache ownership and invalidation;
    - publication of diagnostics toward the editor.

    ## Failure and retry law
    Every failure must state:
    - whether it is retryable;
    - whether rollback is required;
    - which artifact or cache entries are invalidated;
    - which editor surfaces must receive focus; and
    - whether evidence capture is still required.

    ## Current posture
    `doc_closed_impl_open` except where the product relay is explicitly marked `partial_live`.
