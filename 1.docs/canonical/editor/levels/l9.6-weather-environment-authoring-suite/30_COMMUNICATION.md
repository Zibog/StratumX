# Communication

## Sends or publishes
- weather profile updates
- lighting/weather preview refreshes
- environment bake/build requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`weather_environment_authoring_suite` may not smuggle state through undocumented callbacks or widget-local caches.
