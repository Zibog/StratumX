# Input Binding and Control Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document makes control and input surfaces searchable at the bridge layer.
It maps editor/tooling asks to legal ingress families and names the observation classes expected back from lower stacks.

## Catalog law
The sdk never owns input truth or possession truth.
It owns typed bridge surfaces only.
Any control ask must be expressible as one or more legal packet or control classes.

## Bridge surface classes
| Surface class | Role | Typical asks |
|---|---|---|
| control_binding_packet | declare or mutate action bindings and contexts | bind fire, map jump, add gamepad axis |
| control_context_packet | request context or focus transitions | enter play context, return to editor, capture cursor |
| control_action_packet | carry named action or analog consequence into runtime | fire, interact, move, look |
| camera_control_packet | express view/camera movement or focus requests | orbit, fly, focus entity, switch camera |
| control_diagnostics_observation | explain conflicts, denial, focus state, and active maps | binding conflict, denied action, active context |
| possession_observation | report active receiver of control consequence | player pawn, editor camera, cinematic camera |

## Required naming law
Control-facing packet names must encode:
- whether they are configuration, request, or action surfaces;
- whether they target editor view, gameplay possession, or a shared legal context; and
- whether they are transient preview or durable authored configuration.

## Current posture
The current archive already has ingress-control law and broader packet catalogs.
This document narrows that surface specifically for queryable control tasks.
Broad live implementation proof remains future evidence work.
