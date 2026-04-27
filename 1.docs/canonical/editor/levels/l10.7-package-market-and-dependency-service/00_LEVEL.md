# L10.7 Package Market and Dependency Service

## Role

Own editor-facing package discovery, package dependency visualization, package enablement policy,
and tool/content mount registration. This level makes the editor modular instead of monolithic.

## Owns

- package registry views for editor users
- package dependency graph surfaces
- package install/enable/disable/uninstall intent handling
- content mount visibility
- editor tool mount descriptors
- package validation and compatibility posture display

## Must Not Own

- artifact building
- raw asset importing
- runtime simulation
- shell layout ownership

## External Dependencies

- tooling index_plane for package/search indexes
- tooling derived_plane for dependency and reverse-dependency projections
- tooling artifact_plane for package artifacts and manifests
- tooling validation_runtime for compatibility checks
- tooling build_runtime for install/build jobs
- tooling release_runtime for distributable package outputs
- sdk L5 package/public handle exposure only
