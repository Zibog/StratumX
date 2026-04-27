# Prefetch and Eviction

## Role

Prefetch and eviction discipline.

## Data Model

Prefetch is legal only with an explicit locality score and visible budget headroom.
Eviction consumes residency decisions and may not invalidate neighboring pages by surprise.

## Canonical Locality Rules

- primary grouping is region/chunk locality;
- one runtime page may not span more than 2 adjacent chunk neighborhoods;
- prefetch without locality evidence is illegal;
- eviction may not violate active hot-set pins.
