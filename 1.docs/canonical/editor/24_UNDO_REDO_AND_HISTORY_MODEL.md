# Editor Undo/Redo and History Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines the editor-wide undo/redo model: command journal posture, reversible operation classes, history segmentation, cross-surface undo safety, and forbidden hidden mutation.

## Journal law
All user-visible mutations must be journaled through legal lower-stack commands and transactions.
Undo/redo is a visible reflection of transaction law, not a private panel mechanism.

## Reversible operation classes
- entity/component creation, deletion, and modification
- hierarchy and transform changes
- prefab apply/revert, unpack, create-variant
- layer/data-layer assignments
- asset import/reimport/conversion requests where reversible contracts exist
- layout and workspace changes
- tool and mode changes when user-visible

## Cross-surface safety
Undoing a committed change must invalidate and refresh every affected surface:
- viewport
- outliner
- content browser
- inspector
- validation
- runtime inspector when attached

## Forbidden
- mutating truth without journaled transaction visibility
- hiding prefab apply/revert in panel-local state
- maintaining private undo stacks inside plugins that bypass package history
