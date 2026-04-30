# Detached Fragment Sleep Merge And Host Absorption Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze the cheap runtime law for fragments, chunks, splinters, shards, clods, and residue that briefly behave like falling objects and then become compact host-owned state.

This document exists so the engine does not accumulate an immortal graveyard of tiny rigid bodies.

## Fragment lifecycle law
A detached fragment is lawful only when it obeys this canonical lifecycle:
`detach -> flight -> contact/bounce -> settle_candidate -> merge_to_host or explicit_cleanup -> retained_summary`

## Fragment truth objects
| Truth object | Meaning |
|---|---|
| `fragment_flight_state` | short-lived transform, velocity, spin, mass bucket, owner material |
| `fragment_contact_state` | bounce count, host candidate, scrape / impact class |
| `fragment_settle_state` | sleep threshold, merge delay, legal host verdict |
| `host_absorption_state` | material bucket, residue bucket, visual seed, cover or occupancy delta |
| `fragment_cleanup_digest` | explicit reason why no retained summary exists |

## Legal fragment classes
- `fragment.splinter_small`
- `fragment.shard_small`
- `fragment.chunk_small`
- `fragment.chunk_medium`
- `fragment.clod_small`
- `fragment.residue_only`

The engine may add more classes, but it may not let every material invent private fragment life rules.

## Flight law
During flight a fragment may own only bounded cheap physics:
- gravity;
- cheap drag;
- limited bounce;
- limited slide;
- bounded lifetime.

It may not request the full expensive constraint contour of engine `53` unless an explicit hero-case law says so.

## Bounce and settle law
A fragment class must freeze:
- `max_bounce_count`;
- `settle_speed_threshold`;
- `settle_spin_threshold`;
- `merge_delay_window`;
- `legal_host_family_set`.

The default posture for non-hero fragments is one or two visible bounces, then merge or cleanup.

## Legal host families
A fragment may merge only into a lawful host family.
Baseline host families are:
- `host.terrain_surface`
- `host.floor_surface`
- `host.rubble_bed`
- `host.debris_collector_patch`
- `host.water_body` when the fragment policy explicitly allows floating / sinking summary

## Host-absorption law
When a fragment merges into a host, the engine must retain a compact summary such as:
- material bucket;
- fragment class;
- residue mass bucket;
- char / wet modifier;
- visual seed;
- optional cover or occupancy delta.

The fragment then ceases to exist as an independent rigid body.

## Visibility law
A fragment merge may use a short visual handoff such as:
- dust puff;
- scrape puff;
- settle fade;
- host-colored blend.

These are derived presentation aids only.
They may not become hidden authority over whether absorption happened.

## Persistence law
Persisted world state must prefer host-owned residue summaries over detached-fragment snapshots.
Only hero fragments or explicitly retained large chunks may survive as independent persisted entities.

## Budget law
A lawful fragment policy must declare:
- max fragments per event;
- max simultaneous awake fragments per cell;
- max merge queue size per cell;
- cleanup policy when the budget would be exceeded.

## Forbidden shortcuts
- keeping all fragments alive until the player leaves the region;
- invisible deletion with no merge, cleanup, or residue reason;
- fragment persistence that duplicates the same matter both as a host residue and as a live entity;
- large numbers of tiny fragments participating in expensive constraint stacks forever.

## Current posture
`document_gold / closes_fragment_sleep_merge_gap`
