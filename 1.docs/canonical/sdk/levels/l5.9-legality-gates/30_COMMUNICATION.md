# Communication

## Publication classes
- publishes gate rows consumed during ingress control admission
- emits explicit deny/warn codes, never hidden side effects

## Synchronization surfaces
- legality gate registry publish surface
- control legality evaluation surface

## Communication law
Communication in `legality_gates` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.
