# Runtime Product State and Savepoint Publication Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
    This file freezes the engine-side runtime truth doctrine for product relay.
    It belongs in `engine/` because the engine owns the final runtime consequence.
    Upper layers may request, route, observe, compare, or visualize this domain.
    Upper layers may not redefine its runtime truth.

    ## Truth objects
    - ProductStateEnvelope
- SavepointPublicationState
- LaunchSessionIdentity

    ## Runtime phases
    - 1. capture state
- 2. publish savepoint
- 3. confirm durability
- 4. return observation

    ## Ingress and egress carriers
    - ProductStateObservation
- SavepointObservation

    ## Diagnostics and failure classes
    - SAVEPOINT_DENIED
- STATE_PUBLICATION_FAILURE

    ## Degradation and fallback law
    Every domain in this file must define:
    - what degrades first under pressure;
    - what may not degrade without explicit verdict;
    - which fallback still preserves truthful operator understanding; and
    - which fallback becomes illegal because it would fake a result.

    ## Forbidden shortcuts
    - editor may not bypass engine truth by presenting local approximations as runtime truth;
    - tooling may not suppress engine denial codes;
    - sdk may not normalize away domain-critical distinctions;
    - a successful observation may not be published if the runtime phase failed and no legal fallback exists.

    ## Evidence obligations
    Promotion beyond `doc_closed_impl_open` requires:
    - at least one runtime path test;
    - one diagnostics capture proving the phase order;
    - one observation sample proving the carrier schema; and
    - one failure sample proving denial publication.

    ## Current posture
    `doc_closed_impl_open` unless explicitly upgraded by the implementation reality ledger.
