# Security And Session

## Role

Session identity, trust, and transport-security posture.

## Session Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| identity bind | explicit session identity class only | fail closed | implicit trust |
| security posture | fixed transport security class | refuse unsupported peer | runtime downgrade |
| driver/config overlays | none in canonical session proof | abort proof capture | hidden vendor/debug overlay |
| session rekey/rebind | bounded and explicit | drop session before ambiguity | hidden identity swap |

## Rule

Security/session posture must be frozen enough to support the benchmark protocol and network legality without introducing undeclared session classes.

## Binding Table

| Binding | Canonical source |
|---|---|
| identity/session legality | `09_GLOSSARY.md`, network law |
| benchmark-proof posture | `STRATUMX_BENCHMARK_PROTOCOL.md` |
| startup role bind | `levels/l4-startup/startup/44_NETWORK_ROLE_SELECTION.md` |

## Illegal Patterns

- runtime downgrade of security posture;
- hidden identity swap after bootstrap;
- debug or vendor overlay posture masquerading as canonical proof posture.

## Security Class Matrix

| Security concern | Canonical requirement | Illegal posture |
|---|---|---|
| peer trust class | explicit and fail-closed | implicit trust inheritance |
| session rebind | explicit and bounded | silent identity swap |
| proof posture | stable capture/config posture | debug or vendor overlay drift |

## Operational Contract

Session security is not a startup-only slogan.
After bootstrap, transport continues to own session identity, trust class, and downgrade refusal.
