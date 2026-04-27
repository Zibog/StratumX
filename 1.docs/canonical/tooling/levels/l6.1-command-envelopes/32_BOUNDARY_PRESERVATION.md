# Boundary Preservation

This contract belongs specifically to the command envelopes level and may not be reused verbatim by another tooling level.


## Forbidden ownership
- derived projections
- preview-only payloads masquerading as commands
- direct editor state ownership

## Preservation rule
`command_envelopes` stays valid only while it keeps those classes out.

## Operational note
This file remains active and package-specific for `l6.1-command-envelopes` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
