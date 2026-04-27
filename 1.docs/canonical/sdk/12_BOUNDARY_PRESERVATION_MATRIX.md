# Boundary Preservation Matrix

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

| Boundary | Must preserve |
|---|---|
| engine L4 -> L5 | public sinks, public snapshots, and public batch sources only; no deep internal leakage |
| L5 -> L6 | typed bridge classes, immutable snapshots, immutable batches, and bounded cursors only |
| L5 -> L6A | indirect consumption through L6-owned context and streams by default |
| L5 -> L7 | bounded status, profile, and artifact-facing data only |
| L5 -> L7A | no direct raw bridge coupling by default |

Any consumer above L5 must receive data in a shape that preserves ownership, budget class, and locality discipline.
