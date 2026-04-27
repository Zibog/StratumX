# Index Plane

## Role
`index_plane` builds searchable and addressable indices over immutable snapshots and declared artifacts.

## Owns
- `index_id`
- `source_snapshot_set`
- `indexed_keyspace`
- `freshness_epoch`
- `rebuild_reason`

## Consumes
- `l6.3-snapshot-plane`
- `l6.6-artifact-plane`
- `l6.8-cache-plane`

## Emits
- lookup indices
- reverse-reference indices
- search and dependency query surfaces

## Never owns
- authority mutation ownership
- preview rendering jobs
- editor panel state
