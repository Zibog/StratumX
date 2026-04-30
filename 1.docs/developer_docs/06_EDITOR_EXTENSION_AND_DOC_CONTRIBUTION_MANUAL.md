# Editor Extension and Documentation Contribution Manual

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **developer guide**.


## Add editor surface

1. find canonical lab owner;
2. define SDK packets if needed;
3. define tooling route;
4. add editor controls;
5. add disabled reasons;
6. add recovery/focus rule;
7. add user docs and troubleshooting.

Editor surfaces may not call engine truth directly unless canon names that edge as lawful.

---

# V34 scripting and plugin boundary decision

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Binding decision

StratumX 1.0 does not include arbitrary gameplay scripting in core. It uses data-driven authoring, command routes, validated imports, and compiled/cooked assets.

Future plugin/scripting hosts may add:

- panels;
- commands;
- importers;
- validators;
- previews;
- assistant skills;
- build/cook hooks.

Plugins may not:

- mutate engine truth directly;
- create hidden runtime state;
- bypass SDK packets;
- bypass tooling validation;
- bypass diagnostics/evidence;
- inject backend-specific graphics calls above backend boundary.
