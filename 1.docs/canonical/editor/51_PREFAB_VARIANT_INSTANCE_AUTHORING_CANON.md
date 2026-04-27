# Prefab, Variant, and Instance Authoring Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes how reusable authored content should be described at the product layer.
The goal is to answer prefab asks without mixing source truth, instance overrides, and runtime copies.

## Required product distinctions
The editor must keep separate:
- prefab source identity;
- prefab instance in a scene;
- variant identity;
- local instance overrides;
- apply/revert semantics;
- unpack semantics.

## Canonical route
`selection/content surface -> tooling prefab/scene authority -> sdk refs/artifact surfaces as required -> scene projections + diagnostics`

## Minimum task set
- create prefab from selection;
- instantiate prefab into a scene;
- create variant from base prefab;
- inspect local overrides;
- apply overrides upward when legal;
- revert overrides;
- unpack instance when allowed.

## Current implementation posture
This workflow is now canonically frozen but current uploaded code anchors examined for this patch did not prove a live prefab route.
Current posture: `specified_not_proved`.
