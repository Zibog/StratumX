# Dependencies

This contract belongs specifically to the l10.f1 extension and automation family and is not interchangeable with another editor family.


## Family dependency posture for `extension_and_automation_family`
- member-local coordination for automation, scripting/hot reload, plugin host, presets, and scaffolds
- member-local coordination for service registries, package/dependency service, diagnostics
- member-local coordination for extension and automation requests
- lower packages only through member-legal public surfaces
- no family-local authority shortcut

## Operational note
This file remains active and package-specific for `l10.f1-extension-and-automation-family` / `20_DEPENDENCIES.md`.

## Scope note
The authority, dependency, and audit meaning of 20 DEPENDENCIES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
