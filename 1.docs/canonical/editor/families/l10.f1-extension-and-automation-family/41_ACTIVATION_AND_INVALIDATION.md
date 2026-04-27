# Activation And Invalidation

This contract belongs specifically to the l10.f1 extension and automation family and is not interchangeable with another editor family.


## Activation posture for `extension_and_automation_family`
- members may warm together when automation, scripting/hot reload, plugin host, presets, and scaffolds and related surfaces share locality or tooling dependencies
- inactive family members may not leave hidden live state

## Invalidation posture
- shared indices, diagnostics, or previews tied to automation, scripting/hot reload, plugin host, presets, and scaffolds, service registries, package/dependency service, diagnostics, extension and automation requests may trigger bounded family refreshes

## Operational note
This file remains active and package-specific for `l10.f1-extension-and-automation-family` / `41_ACTIVATION_AND_INVALIDATION.md`.

## Scope note
The authority, dependency, and audit meaning of 41 ACTIVATION AND INVALIDATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
