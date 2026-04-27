# build_validation_release_suite Level

Canonical layer: `build_validation_release_suite`
Activation class: `warm-suite`.

## Owns
- validation dashboards
- broken-reference and missing-dependency surfaces
- bake/build target views
- packaging presets and release runbooks
- artifact and closure reports

## Consumes
- diagnostics/build/release surfaces
- service status
- validation results

## Emits
- validation scans
- bake/build/release requests
- checklist and remediation requests

## Never owns
- validation truth
- build truth
- release truth
