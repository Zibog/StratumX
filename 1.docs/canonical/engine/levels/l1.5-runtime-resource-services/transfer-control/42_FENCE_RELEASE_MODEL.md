# Fence Release Model

## Role

Fence-bound resource release.

## Data Model

Transfer-backed resources remain quarantined until fence-visible completion.
Early release, hidden lifetime extension, and silent readback are illegal.

## Canonical Release Law

- publishable visibility requires decode complete + staging complete + upload submitted + fence-visible completion;
- `quarantined` resources may not be reclassified before the grace window frozen by residency law;
- readback in low-latency presentation path is illegal outside explicit diagnostics windows.
