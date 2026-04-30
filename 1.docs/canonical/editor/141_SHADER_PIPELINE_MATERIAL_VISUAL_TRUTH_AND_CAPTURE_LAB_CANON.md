# Shader Pipeline Material Visual Truth And Capture Lab Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define the editor lab for shader variants, material visual channels, backend shader targets, and captured proof.

## Required controls
- select material family;
- inspect shader variant;
- inspect backend target output;
- recook shader variant;
- show compile diagnostics;
- bind fallback shader;
- capture material proof frame;
- compare material visual truth.

## Required fields
- shader family;
- material family;
- backend target;
- source hash;
- target hash;
- pipeline key;
- binding layout id;
- compile status;
- first failure;
- fallback variant.

## Law
A material may look degraded. It may not lose truth silently.

## Current posture
`document_gold / shader_material_lab_defined / implementation_open`


---
# V32 Editor Lab Closure: Shader/Material

## Required sections
Shader family list, selected backend target, shader variant key, compile/cook status, reflection/binding summary, pipeline cache status, material channel table, missing texture/material fallbacks, capture metadata preview.

## Material channel inspector
Base color, normal, roughness, metal/specular, AO, emissive, opacity/cutout, wetness placeholder, char/burn placeholder, damage/reveal placeholder, missing/fallback state.
