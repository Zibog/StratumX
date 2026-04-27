# Editor Visual Language Theme Iconography And Motion Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the visual identity of the editor so the product reads as one premium deep-work instrument rather than a pile of correct but unrelated panels.

## Theme identity law
The canonical editor theme is a **deep-space black** system with controlled violet-blue spectral accents.
It must read as:
- cinematic without becoming noisy;
- high-contrast without depending on pure white;
- minimal without becoming sterile;
- dense enough for serious work without becoming hostile.

## Canonical color system
### Core shell colors
- shell void: near-black with a cold blue-violet bias
- panel base: slightly raised from shell void so dock boundaries remain readable
- elevated chrome: one additional value step above panel base for toolbars and selected tabs
- hard separators: restrained, low-opacity cool separators; no chalky bright lines

### Text colors
- primary text: bright cool-gray, not white
- secondary text: softer desaturated cool gray
- tertiary text: dim neutral-cool gray for metadata and hints
- disabled text: low-contrast but still legible under accessibility rules

### Status accents
- success: restrained emerald-cyan
- warning: muted amber
- danger: deep coral-red
- info: electric cyan
- assistant / generative: controlled violet
- active selection / route focus: blue-violet highlight ring, never neon paint spill

## Contrast law
The theme must preserve legibility for long sessions.
This means:
- no pure white body text on the shell by default;
- no tiny light text over overly saturated accent chips;
- no primary controls that disappear into the black surface;
- no “all panels same color” collapse.

## Typography law
The editor typography system must use:
- a highly readable UI sans for labels, controls, and inspector fields;
- larger section titles for shell orientation;
- medium-weight emphasis for selected panels and active stages;
- monospaced text only for ids, paths, technical overlays, routes, and diagnostics.

Typography must support:
- compact inspector density;
- large readable stage titles;
- stable tab labels with no jitter on selection;
- code-like overlays in the viewport without swallowing the scene.

## Iconography law
Icons are minimal, geometric, and calm.
They must:
- remain readable at small sizes;
- prefer line or duotone-light silhouettes over filled toy shapes;
- communicate category first, detail second;
- stay consistent across world, terrain, material, audio, simulation, and capture families.

Forbidden icon posture:
- cartoon metaphor overload;
- random third-party icon mixes;
- per-panel stylistic drift;
- over-detailed glyphs that blur in compact layouts.

## Motion law
Micro-motion is allowed only to reinforce understanding.
Canonical uses:
- panel expand/collapse easing;
- focus handoff glow or outline;
- selection pulse in the outliner or viewport;
- timeline scrub feedback;
- subtle assistant activity indicators;
- dock attach/detach confirmation motion.

Forbidden motion posture:
- decorative looping animations;
- long transitions that slow heavy work;
- splashy screen wipes between stages;
- attention theft during simulation or certification review.

## Density law
The editor must support three densities:
- relaxed review;
- standard production;
- compact expert.

All densities must preserve:
- toolbar readability;
- inspector scannability;
- stage-strip identity;
- bottom-strip tab recognition.

## Accessibility law
The theme must remain usable for long low-light sessions.
This means:
- non-color cues for warnings and errors;
- keyboard focus rings visible on dark surfaces;
- scalable text;
- no crucial state that is only visible in faint violet-on-black subtlety.

## Cross-reference
This visual language is consumed by:
- `114` shell topology and docking law;
- `115` viewport cockpit law;
- `117` stage strip law;
- `129` world conveyor workspace law;
- `130` viewport timeline and creation-bar law;
- `131` constellation navigator identity.
