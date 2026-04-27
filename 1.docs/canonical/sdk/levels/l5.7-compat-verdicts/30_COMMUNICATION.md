# Communication

## Publication classes
- receives evaluation inputs from legality/transport validation
- publishes verdict records for downstream acceptance or rejection

## Synchronization surfaces
- compatibility evaluation surface
- compatibility verdict publication surface

## Communication law
Communication in `compat_verdicts` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.
