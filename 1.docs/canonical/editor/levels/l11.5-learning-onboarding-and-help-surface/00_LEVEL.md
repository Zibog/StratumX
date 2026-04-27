# learning_onboarding_and_help_surface Level

Canonical layer: `learning_onboarding_and_help_surface`
Activation class: `cold-ops`.

## Role
learning onboarding and help surface is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- embedded help, tutorials, playbooks, onboarding checklists, contextual guidance

## Consumes
- shell context, suite context, documentation registries

## Emits
- help reveal requests, onboarding progress requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- project truth
