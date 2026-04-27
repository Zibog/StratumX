# Boundary Preservation

This contract belongs specifically to the l8.f3 command and interaction family and is not interchangeable with another editor family.


## Must stay out of `command_and_interaction_family`
- member-level truth ownership for command palette, shortcuts, interaction routing, and mode/context switching, shortcut maps, tool contexts, shell hooks, normalized product intents
- lower-package mutable authority
- unrelated editor domains

## Operational note
This file remains active and package-specific for `l8.f3-command-and-interaction-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
