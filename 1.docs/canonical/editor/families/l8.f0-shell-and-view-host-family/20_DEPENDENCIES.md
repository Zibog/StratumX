# Dependencies

This contract belongs specifically to the l8.f0 shell and view host family and is not interchangeable with another editor family.


## Family dependency posture for `shell_and_view_host_family`
- member-local coordination for editor shell, workspace layouts, detached hosts, and primary panel/view anchoring
- member-local coordination for shell frame, layout state, panel registries
- member-local coordination for view host activation, layout requests
- lower packages only through member-legal public surfaces
- no family-local authority shortcut

## Operational note
This file remains active and package-specific for `l8.f0-shell-and-view-host-family` / `20_DEPENDENCIES.md`.

## Scope note
The authority, dependency, and audit meaning of 20 DEPENDENCIES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
