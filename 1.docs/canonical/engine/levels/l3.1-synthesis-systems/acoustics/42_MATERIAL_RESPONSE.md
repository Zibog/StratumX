# Material Response

## Role

Acoustic material-response consumption.

## Data Model

Acoustics reads material reaction profiles from `engine_material` and may not redefine response ownership.

## Response Matrix

| Item | Canonical law |
|---|---|
| lookup source | `engine_material` only |
| lookup shape | compact-id and flattened reaction rows |
| fallback | deterministic material fallback law |
| duplicated ownership | illegal |

## Canonical Law

- acoustic response lookup uses compact ids and flattened reaction profiles only;
- acoustics may not build a second descriptor graph for material ownership;
- fallback response uses the deterministic material fallback law.
