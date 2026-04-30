# Documentation Spine and Role Separation Canon

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **documentation governance canon**.


## Purpose

StratumX documentation is split into role-specific branches so the canon stays clean while developers and users receive practical manuals.

## Branch roles

| Branch | Role | May define law? | Audience |
|---|---|---:|---|
| `canonical/` | Internal law and architecture truth | yes | engine architects, code agents, maintainers |
| `developer_docs/` | How to implement code according to canon | no, interprets canon | programmers and coding agents |
| `user_docs/` | How to use editor/runtime tools | no | designers, technical artists, operators |
| `api_reference/` | Stable structures, packets, commands, routes, error codes | no, references canonical schema law | SDK/tooling/editor integrators |
| `tutorials/` | Step-by-step proof workflows | no | humans and execution agents |
| `troubleshooting/` | Failure triage and recovery | no, maps canonical errors to recovery | all operators |

## Non-redefinition rule

If a branch conflicts with `canonical/`, the canonical document wins. Practical docs must link conceptually to canonical owners and may not invent new authority, new packet semantics, or new success criteria.

## Completion target

A domain is considered Unreal-style documented when it has:

1. canonical law;
2. developer implementation guide;
3. user/editor guide;
4. API reference rows;
5. at least one tutorial;
6. troubleshooting page;
7. acceptance/evidence criteria.

## V33 domain targets

| Domain | Target |
|---|---|
| Graphics | 100% document-gold for practical showable frame route. |
| Asset/model pipeline | 100% document-gold for import/cook/bind/troubleshoot. |
| Audio authoring | 100% document-gold for authoring/bank/material sound/audition. |
| Netcode | Minimum 75% document readiness: enough to implement cleanly, not yet shipped-maturity. |

## Non-negotiable rules

| Rule | Meaning |
|---|---|
| No fake success | A route may return `NotImplemented`, `Blocked`, `Unavailable`, or `Unsupported`, but it may not return success for an unimplemented behavior. |
| One owner | Every truth object has exactly one owner layer. Other layers may hold handles, DTOs, views, or cached projections only. |
| Observable failure | Every failure family must produce an error code, disabled reason, recovery hint, and retained diagnostic packet. |
| Editor honesty | The editor may expose the route, preview, capture, or recovery action, but it must not mutate engine truth except through lawful SDK/tooling ingress. |
| Evidence or it did not happen | Release-grade claims require capture, compare, replay, or retained diagnostic evidence. |

## Required completion shape

Every implementation derived from this document must include:

1. owner module or crate;
2. public data contracts;
3. lifecycle stages;
4. failure and disabled reason codes;
5. editor/tooling/SDK contact points;
6. quality tests and negative-path tests;
7. retained evidence artifacts for certification routes.

---

# V34 scripting and plugin boundary decision

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Binding decision

StratumX 1.0 does not include arbitrary gameplay scripting in core. It uses data-driven authoring, command routes, validated imports, and compiled/cooked assets.

Future plugin/scripting hosts may add:

- panels;
- commands;
- importers;
- validators;
- previews;
- assistant skills;
- build/cook hooks.

Plugins may not:

- mutate engine truth directly;
- create hidden runtime state;
- bypass SDK packets;
- bypass tooling validation;
- bypass diagnostics/evidence;
- inject backend-specific graphics calls above backend boundary.
