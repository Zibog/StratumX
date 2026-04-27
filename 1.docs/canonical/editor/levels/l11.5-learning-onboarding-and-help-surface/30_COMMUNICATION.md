# Communication

## Sends or publishes
- help topic changes
- onboarding progress updates
- doc/open-command requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`learning_onboarding_and_help_surface` may not smuggle state through undocumented callbacks or widget-local caches.
