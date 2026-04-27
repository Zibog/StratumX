# Release Seal Freeze And Reality Review Workbench Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Provide the editor-facing surface for final document sealing and honest readiness review.

## Mandatory panels
- index sync
- ledger sync
- coverage sync
- readiness sync
- evidence sync
- open truth review
- first-playable readiness
- seal verdict

## Mandatory verdict classes
`seal.ready_in_docs`, `seal.blocked_by_sync`, `seal.blocked_by_open_truth`, `seal.blocked_by_missing_evidence`

## Required inputs
- `packet.release_seal_review.v1`
- `packet.first_playable_readiness.v1`
- active index digests
- open-truth ledger rows
- evidence registry refs

## Focus law
A failed row must open the exact stale or blocked surface instead of a generic dashboard:
index file, ledger row, coverage row, lab, or evidence review pane.

## Honesty rule
This workbench may declare `document_gold`.
It may not auto-upgrade that verdict to runtime gold.
