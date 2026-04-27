# Boundary Preservation

## Must stay out of this level
- runtime simulation truth
- editor shell ownership
- hidden world-state mirrors

## Preservation rule
`simulation_ai_authoring_suite` exists to keep this editor concern local instead of leaking it into sdk, tooling, or another editor level.

## Operational note
This file remains active and package-specific for `l9.5-simulation-ai-authoring-suite` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
