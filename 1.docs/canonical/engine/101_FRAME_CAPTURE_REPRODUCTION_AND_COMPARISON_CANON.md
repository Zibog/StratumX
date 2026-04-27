# Frame Capture Reproduction and Comparison Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines canonical capture and replay of frames for diagnostics and certification.

## Runtime truth objects
- FrameCaptureState, CaptureComparisonState

## Runtime phases
- request capture -> serialize -> compare/replay -> publish deltas

## Diagnostics and evidence
- diagnostics must expose ids, reasons, and policy choices
- degradation must be explicit and traceable
- forbidden shortcuts: no editor-local or tooling-local state may impersonate runtime truth

## Current posture
- document posture: `document_gold`.
- implementation posture: `doc_closed_impl_open` unless proof says otherwise.
