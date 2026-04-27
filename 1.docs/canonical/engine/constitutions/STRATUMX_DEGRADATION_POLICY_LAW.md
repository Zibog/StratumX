# STRATUMX_DEGRADATION_POLICY_LAW

## 1. Purpose

This document defines canonical degradation policy for StratumX.

## 2. Degradation Principles

- critical simulation progression may not be degraded into non-progression;
- lower stack and world truth may not be bypassed as a degradation strategy;
- runtime resource services may adapt budgets, cadence, and pressure response, but may not be degraded away when mandatory;
- selected network runtime services may adapt rate, interest scope, and budget, but may not be degraded away while their selected role remains legal;
- service layers may be deferred, thinned, or disabled by policy when legal;
- degradation decisions must be explicit, observable, and reversible by legal startup/configuration.

## 3. Degradation Classes

### 3.1. Non-degradable core
May not be degraded away:

- lower stack;
- `engine_material`;
- runtime family;
- mandatory runtime resource services;
- `engine_kinetics`;
- `engine_field`;
- `engine_agents`;
- `engine_startup`.

### 3.2. Role-selectable services
May be disabled only by legal profile or role selection:

- `engine_transfer_control`
- `engine_net_transport`
- `engine_net_sync`
- `engine_net_latency`

### 3.3. Legally degradable service layers
May be degraded under policy:

- `engine_inference`
- `engine_generation`
- `engine_imaging`
- `engine_acoustics`
- `engine_content`

## 4. Canonical Degradation Order

### 4.1. `interactive-60`
1. optional diagnostics extras;
2. ambience and decorative acoustics;
3. optional generation and inference services;
4. optional prefetch and warm residency;
5. imaging quality classes that do not break low-latency legality;
6. publication of cosmetic-only network hints.

### 4.2. `listen-host-60`
1. optional diagnostics extras;
2. ambience and decorative acoustics;
3. optional prefetch and optional transfer warmup;
4. cosmetic-only remote publication;
5. optional generation and inference services;
6. imaging quality classes that do not break local low-latency legality.

### 4.3. `headless-20`
1. optional diagnostics extras;
2. optional generation and inference services;
3. non-authoritative export verbosity;
4. warm residency and background verification depth.

## 5. Illegal Degradation Decisions

- skipping apply for world progression;
- dropping critical simulation families;
- disabling mandatory resource services in legal assemblies that require them;
- disabling selected network runtime services without changing the legal role selection;
- replacing world truth with generated outputs;
- creating a second runtime authority.

## 6. Observability

All degradation decisions must be emitted through runtime diagnostics and observability channels.
