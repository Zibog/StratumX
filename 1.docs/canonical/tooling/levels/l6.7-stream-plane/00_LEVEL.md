# Stream Plane

## Role
`stream_plane` carries bounded event and status streams that are live enough for observers but not authoritative truth by themselves.

## Owns
- `stream_event_id`
- `stream_kind`
- `source_scope_id`
- `ordering_cursor`
- `retention_window`

## Consumes
- `l6.0-authority-core`
- `l6.9-budget-runtime`

## Emits
- diagnostic streams
- preview status streams
- build and release progress streams

## Never owns
- long-lived truth ownership
- hidden replay archives beyond declared retention
- editor widget state
