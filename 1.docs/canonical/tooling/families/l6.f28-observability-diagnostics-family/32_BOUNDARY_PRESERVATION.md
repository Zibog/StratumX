# Boundary Preservation

This contract belongs specifically to the l6.f13 observability diagnostics family and describes family-only coordination.


## Must remain outside `observability_diagnostics_family` ownership
- member-internal mutable authority rows for observability views, logs, probes, metrics dashboards, validation visibility, and diagnostics routing, authority-facing minimal truth: observability-session refs only, snapshot classes: observability snapshots, index classes: diagnostics lookup indices
- unrelated domain truth not declared in this family
- editor widget/layout state

## Operational note
This file remains active and package-specific for `l6.f28-observability-diagnostics-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
