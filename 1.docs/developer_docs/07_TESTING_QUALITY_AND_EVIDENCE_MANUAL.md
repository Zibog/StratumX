# Testing, Quality, and Evidence Manual

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **developer guide**.


## Test placement

Integration, property, matrix, proof, replay, import, backend, network, and editor workflow tests belong in `7.quality`.

Production crates may contain only tiny local invariants. No hidden integration tests in production crates.

## Evidence suites

Each release-grade domain must produce retained evidence: capture artifact, packet trace, replay, diff, import verdict, desync trace, or audio audition evidence.

---

# V34 mandatory gold gates

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Code gates

- workspace membership gate;
- layer dependency gate;
- file size gate;
- test placement gate;
- orphan active crate gate;
- stub crate gate;
- backend type leak gate;
- source asset runtime leak gate;
- fake success gate.

## Documentation gates

- stack marker gate;
- branch role gate;
- broken link gate;
- active index gate;
- thin doc warning;
- orphan doc warning;
- tutorial/reference/troubleshooting coverage gate for every user-visible system.

## Asset gates

- DCC bridge configured or exact blocker;
- source/cooked split proven;
- import route evidence;
- material conversion verdict;
- collision/LOD verdict;
- runtime package proof.
