# Editor Activation and Cold Surface Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Surface temperature classes
### Hot surface
Visible, focused, and budget-privileged.
Examples: active viewport, focused outliner, focused inspector, active runtime inspector.

### Warm surface
Visible or recently used, but not focus-critical.
Examples: validation dock, background queue panel, hidden-but-pinned timeline.

### Cold surface
Closed or dormant; retains no heavy resources.
Examples: inactive suites, detached but closed secondary windows, dormant plugin panels.

## Activation law
- panels, suites, and services activate lazily
- activation must not block focused interaction
- secondary previews and dormant suites may warm, but must cool under pressure
- runtime attach state must cool immediately on session end or authority loss

## Forbidden hidden background ownership
The editor must not maintain hidden jobs, hidden caches, hidden mutation queues, or hidden runtime bindings.
Every long-lived background concern must have a visible status surface somewhere in the product.
