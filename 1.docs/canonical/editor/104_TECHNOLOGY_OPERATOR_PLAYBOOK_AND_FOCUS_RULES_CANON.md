# Technology Operator Playbook And Focus Rules Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose the canonical no-leak operator chain without stealing runtime truth from engine.

## Command inventory
- `open.playbook.chain`
- `focus.next_action`
- `route.to_owner_lab`
- `freeze.review.start`

## Required focus fields
- `playbook_step_id`
- `owning_route_id`
- `active_blocker_family`
- `retained_bundle_id`
- `next_legal_action_id`

## Chain law
This surface must expose the root chain from `73_OPERATOR_CHAIN_CERTIFY_FREEZE_AND_EVIDENCE_ROUTE_CANON.md` exactly as:
`project -> content -> world -> gameplay -> certify -> freeze`.

## Focus rules
- success -> owning target lab;
- failure -> `80`;
- pressure -> `74` or `81`;
- evidence gap -> `105`;
- freeze blocker -> `109`.

## Evidence duty
If this lab participates in certification, it must point to one retained artifact id, one terminal blocker or pass code, and one next legal action whenever a failure occurs.
