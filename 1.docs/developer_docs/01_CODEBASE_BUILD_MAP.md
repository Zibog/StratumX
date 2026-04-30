# Codebase Build Map

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **developer guide**.


## Layer rule

`2.engine` owns runtime truth. `3.sdk` owns packets/DTO/contracts. `4.tooling` owns routes/conveyors/evidence. `5.editor` owns operator surfaces. `6.apps` owns thin launch hosts. `7.quality` owns tests and proof suites.

## Add-code checklist

1. identify canonical owner;
2. create or extend the owner crate;
3. expose SDK packets only if data crosses a layer;
4. expose tooling route only if an operator action or process exists;
5. expose editor UI only through tooling/SDK route;
6. write tests in `7.quality` unless they are tiny local unit invariants;
7. update API reference and troubleshooting if a new packet, command, route, or error exists.
