# editor_shell Level

Canonical layer: `editor_shell`
Activation class: `warm-shell`.

## Role
The editor shell is the canonical product/service surface for the visible editor frame.
It exists above the lower-stack authority layers and owns the workspace frame, dock host, toolbar, menus, status strip, and command hubs.

## Owns
- menu bar
- main toolbar and play/debug toolbar
- tab host and dock composition
- status strip and notifications
- workspace frame
- shell routing and command hubs

## Consumes
- lower-stack status signals
- public queue and diagnostics state
- workspace session-safe refs where needed
- command palette requests
- assistant/diagnostics/build surface presence

## Emits
- shell actions
- workspace change requests
- panel/view host activation
- product-level intents and command dispatch

## Never owns
- lower-stack truth
- lower-stack transactions
- project/game truth
