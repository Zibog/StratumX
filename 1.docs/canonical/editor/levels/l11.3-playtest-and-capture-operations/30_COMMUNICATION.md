# Communication

## Sends or publishes
- play/stop/step requests
- PIE Attach requests
- runtime watch and capture publications

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`playtest_and_capture_operations` may not smuggle state through undocumented callbacks or widget-local caches.
