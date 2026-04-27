# Command Envelopes

## Role
`command_envelopes` defines canonical command envelope schemas for every tooling mutation path before transaction materialization.

## Owns
- `command_envelope_id`
- `command_kind`
- `target_ref_set`
- `issuer_session_id`
- `mutation_intent_digest`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.14-tool-task-requests`

## Emits
- validated command envelopes
- transaction-ledger materialization inputs
- command reject records

## Never owns
- derived projections
- preview-only payloads masquerading as commands
- direct editor state ownership
