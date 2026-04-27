# Assistant to Tooling Lowering and Apply Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes how assistant requests become legal tooling actions.
The assistant is allowed to help.
It is not allowed to become hidden authority.

## Canonical assistant route
`user ask -> editor assistant surface -> tooling assistant runtime -> proposal -> legality/safety check -> preview or dry-run if required -> explicit apply/revert path -> diagnostics and evidence publication`

## Allowed outcome classes
- `proposal_only`
- `preview_ready`
- `apply_ready`
- `applied`
- `reverted`
- `denied`
- `deferred`

## Main laws
- assistant may not write engine truth directly;
- assistant may not skip tooling authority or legality gates;
- assistant must preserve explicit apply/revert semantics;
- assistant must report when a target route is only specified or deferred;
- assistant must cite the current posture from the status ledgers.

## Typical ask examples
- import and place a model;
- create a material stack and assign it;
- prepare a weather/fire test scene;
- run validation or build flows;
- stage a combat or destruction preview.
