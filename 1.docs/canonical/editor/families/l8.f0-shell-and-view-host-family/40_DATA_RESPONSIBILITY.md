# Data Responsibility

This contract belongs specifically to the l8.f0 shell and view host family and is not interchangeable with another editor family.


## Family-scoped data for `shell_and_view_host_family`
- coordination data for editor shell, workspace layouts, detached hosts, and primary panel/view anchoring
- coordination data for shell frame, layout state, panel registries
- coordination data for view host activation, layout requests

## Note
Family-scoped data stays descriptive and coordinating; member levels keep the operational truth.

## Operational note
This file remains active and package-specific for `l8.f0-shell-and-view-host-family` / `40_DATA_RESPONSIBILITY.md`.

## Scope note
The authority, dependency, and audit meaning of 40 DATA RESPONSIBILITY is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
