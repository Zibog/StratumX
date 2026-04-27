# Response Profiles

## Role

Material response-profile mapping.

## Data Model

Response profiles map compact material ids to reaction rows consumed by kinetics, field, acoustics, and imaging.

## Response Matrix

| Item | Canonical law |
|---|---|
| profile identity | compact immutable id |
| row shape | flattened reaction row |
| mutation | only through authoritative apply or legal reload path |
| duplicated ownership | illegal |

## Canonical Law

- response profiles are immutable at runtime unless changed through authoritative apply or legal content reload paths;
- response-profile lookup must remain O(1)-like and cache-friendly for hot paths;
- material consumers may not duplicate ownership of response law.
