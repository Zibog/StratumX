# Communication

This communication contract belongs specifically to the command envelopes tooling level.

## Sends or publishes
- validated command envelopes
- transaction-ledger materialization inputs
- command reject records

## Required posture
- sender and receiver scopes are explicit for command envelopes work
- published records such as command_envelope_id, command_kind remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for command envelopes may not be copied to another level without changing operational meaning.
