# Profile Selection

## Role

Startup profile bind.

## Profile Matrix

| Profile | Required bind inputs | Required guarantees | Illegal posture |
|---|---|---|---|
| `interactive-60` | hardware-floor legality + realtime role legality | bind frozen before runtime ownership | runtime-selected implicit profile |
| `listen-host-60` | hardware-floor legality + realtime host role legality | host burden frozen before launch | post-launch host widening |
| `headless-20` | hardware-floor legality + headless role legality | no local presentation ownership | headless borrowing realtime reserve |

## Rule

Profile selection is startup-owned, explicit, and complete before runtime ownership begins.

## Failure Posture

Profile mismatches against hardware floor, role bind, or resource-service legality fail closed before runtime ownership begins.

## Illegal Patterns

- implicit profile fallback;
- post-bootstrap profile widening;
- startup selecting a profile from service-owned policy.

## Selection Matrix

| Selection concern | Required law | Illegal shortcut |
|---|---|---|
| hardware floor bind | explicit before launch | runtime-side profile pick |
| role compatibility check | explicit before launch | silent fallback role |
| service compatibility check | explicit before launch | post-launch profile repair |

## Local Operating Law

Startup profile selection is a hard bind surface.
It exists to prevent runtime-side improvisation around profile ceilings.
