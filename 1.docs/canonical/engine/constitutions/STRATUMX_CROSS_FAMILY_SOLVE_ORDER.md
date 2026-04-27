# STRATUMX_CROSS_FAMILY_SOLVE_ORDER

## 1. Purpose

This document freezes legal same-tick interaction order between `engine_kinetics`, `engine_field`, and `engine_agents`.
It also freezes the size and shape of legal same-tick bridge traffic so solve order cannot be obeyed while bridge volume still explodes.
Numeric constants are authored in `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` and `STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`.

## 2. Authoritative Compute Order

Inside one runtime tick:

1. immutable read snapshot and regional activation are frozen;
2. `engine_agents` may emit intent and squad-actuation products from read-only state;
3. `engine_kinetics` solves local trajectories, contacts, and immediate physical actuation;
4. `engine_field` solves local field evolution using legal same-tick physical summaries and prior committed field state;
5. deferred agent-world consequences consume staged results only;
6. same-tick bridge products are staged;
7. world apply publishes the authoritative integrated state.

## 3. Same-Tick Bridge Law

| Bridge | Same-tick legality | Notes |
|---|---|---|
| `agents -> kinetics` | legal | actuation, target steering, and bounded avoidance hints only |
| `kinetics -> field` | legal | impact energy, contact material, fracture summary, heat source, and fluid ingress summary only |
| `field -> kinetics` | legal | bounded drag, buoyancy, friction, damping, or burn-state coefficients only |
| `agents -> field` | deferred preferred | same-tick legal only through bounded authored triggers, not full field solve control |
| `field -> agents` | deferred | perception/evaluation consequences consume staged field outputs, not live field ownership |
| `kinetics -> agents` | deferred | combat/damage and structural consequences consume staged outputs |

## 4. Immediate vs Deferred Law

- immediate bridges are legal only when they are bounded coefficients, summaries, or actuation descriptors;
- rich consequence payloads are deferred to staged/apply-visible products;
- no family may pull another family into a second hidden solve pass in the same tick.

## 5. Bridge Record Classes

| Record class | Ceiling |
|---|---|
| coefficient record | <= 32 bytes |
| actuation or steering hint | <= 64 bytes |
| impact or fracture summary | <= 128 bytes |
| local structural event | <= 256 bytes |
| opaque blob | illegal |

## 6. Bridge Payload Ceilings

- same-tick bridge payload: <= 64 KiB per family-pair per region per tick;
- aggregate same-tick bridge payload: <= 512 KiB per tick on `interactive-60` and `listen-host-60`, and <= 768 KiB per tick on `headless-20`;
- aggregate same-tick bridge record count: <= 4,096 records per authoritative tick.

## 7. Illegal Patterns

- whole-world field recomputation triggered by one local kinetics event;
- far-tier agents demanding near-tier kinetics cadence without promotion;
- hidden same-tick double-solve loops between families;
- unlimited bridge blobs disguised as "summary" payloads.
