# Communication

## Sends or publishes
- track edit requests
- shot and camera rig updates
- timeline preview/playback publications

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`animation_cinematics_authoring_suite` may not smuggle state through undocumented callbacks or widget-local caches.
