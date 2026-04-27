# STRATUMX_CONSTITUTION_PRECEDENCE_RULE

## 1. Purpose

This document defines canonical precedence between root documents, constitutional documents, level documents, and crate documents.

## 2. Canonical Order of Authority

Precedence is:

1. root canonical stack identity
2. constitutional operational law
3. level definitions
4. crate-local detail
5. element-local detail

## 3. Root Canonical Stack Identity

The following are owned by root documents and may not be redefined elsewhere:

- level names;
- crate names;
- family names;
- canonical stack order;
- alias mappings;
- transition and preservation mappings;
- root summary maps.

## 4. Constitutional Operational Law

Constitutional documents are the only normative source for:

- execution law;
- threading law;
- data/state law;
- progression law;
- naming freeze;
- API contract law;
- canonical shape law;
- configuration law;
- enablement legality;
- observability law;
- degradation law;
- persistence law;
- compatibility law;
- testing law;
- performance law;
- workspace/repository law.

## 5. Level and Crate Documents

Level and crate documents may define:

- local role;
- local dependencies;
- local communication;
- local threading posture;
- local element structure.

They may not redefine:

- stack order;
- runtime authority law;
- global threading ownership;
- persistence compatibility law;
- naming freeze.

## 6. Summary vs Normative Documents

- `07_THREADING_MODEL.md` is a summary map.
- `08_EXECUTION_FLOW.md` is a summary map.
- constitutional documents are normative.
- if a summary map and a constitutional document differ, the constitutional document wins.

## 7. Conflict Resolution

If a conflict appears:

1. preserve root stack identity;
2. preserve constitutional law;
3. update lower-precedence documents;
4. update glossary and alias mappings;
5. record intentional boundary changes in transition and preservation maps.

## 8. Change Discipline

A change is incomplete unless it updates every affected document class:

- root stack docs where names or stack shape changed;
- constitutional docs where operational law changed;
- level/crate docs where local boundary changed;
- glossary and alias docs where terminology changed.
