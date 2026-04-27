# Document Rules

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## 1. Purpose

This document defines how canonical engine documents are written, updated, and prioritized.

## 2. Canonical Document Classes

### 2.1. Root canonical documents
Root documents define stack shape, taxonomy, package structure, navigation, glossary, role map, dependency map, communication map, and transition history.

### 2.2. Constitutional documents
Constitutional documents define engine-wide operational law, governance law, compatibility law, API contract law, testing law, performance law, and canonical shared shapes.

### 2.3. Implementation documents
Implementation documents define implementation order, implementation handoff law, starter compile skeletons, first live milestones, and minimal contract surfaces. They may not redefine architecture, taxonomy, or constitutional law.

### 2.4. Level and crate documents
Level and crate documents define level-local and crate-local canonical boundaries.

## 3. Precedence Rule

When documents overlap, priority is:

1. constitutional documents for engine-wide operational, governance, compatibility, API contract, canonical shape, testing, and performance law;
2. root canonical documents for stack, taxonomy, naming, package structure, navigation, and glossary;
3. level and crate documents for local boundaries and roles;
4. implementation documents for build order, handoff, starter surfaces, and milestone sequencing.

## 4. Normative vs Explanatory Rule

- constitutional documents are normative for engine-wide law;
- root canonical documents are normative for stack shape, terminology, package structure, and canonical navigation;
- level and crate documents are normative for local boundaries and roles;
- implementation documents are normative for implementation sequencing, handoff, and starter delivery, but not for stack redefinition;
- summary maps remain explanatory unless they define canonical stack shape or approved navigation.

## 5. Update Rule

### 5.1. Update root docs when:
- stack shape changes;
- canonical level or crate names change;
- umbrella terms are introduced or changed;
- glossary or alias coverage changes;
- package structure changes.

### 5.2. Update constitutional docs when:
- execution, threading, state, persistence, configuration, observability, testing, performance, degradation, governance, API contract law or canonical shape law changes.

### 5.3. Update implementation docs when:
- build order changes;
- implementation handoff rules change;
- first compile skeleton changes;
- first live milestone changes;
- minimal public contract starter surfaces change.

### 5.4. Update level/crate docs when:
- a local role, dependency, communication boundary, threading boundary, or element definition changes.

## 6. Naming Rule

Each document must use canonical names exactly as frozen by the naming and terminology freeze.

## 7. One Topic Rule

Each document must have one primary job. Wide coverage is legal only if the document defines one clear law, one clear package map, or one clear delivery plan.

## 8. Historical Reference Rule

Historical file names may appear only when explicitly marked as historical literal references. They must not look like live canonical paths.
