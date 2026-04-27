# l11.2-asset-gate-and-approval-surface Libraries

## Allowed library classes
- approval gate descriptors
- review state and ownership helpers
- publish/merge eligibility adapters
- gate escalation and waiver guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l11.2-asset-gate-and-approval-surface` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l11.2-asset-gate-and-approval-surface` and must stay specific enough to support implementation work without reinterpretation.
