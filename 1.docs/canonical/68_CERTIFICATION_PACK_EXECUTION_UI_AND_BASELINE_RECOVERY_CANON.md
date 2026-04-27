# Certification Pack Execution UI And Baseline Recovery Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Close the certification loop for heavy-domain packs.
Define who launches a pack, where failure is rendered, how baseline protection works, how regression is reviewed, and which recovery action is legal next.

## Operator flow matrix
| Domain / pack family | Launch surface | Compare surface | Failure surface | Baseline surface | Signoff surface | First legal recovery action |
|---|---|---|---|---|---|---|
| hydrology and persistence | `editor/66`, `editor/77`, `editor/81` | `editor/66`, `editor/77`, `editor/103` | `editor/66`, `editor/100` | `editor/103`, `editor/105` | `editor/87`, `editor/109` | restore last-good hydrology checkpoint baseline |
| storm and atmosphere | `editor/67`, `editor/81`, `editor/94` | `editor/67`, `editor/94`, `editor/103` | `editor/67`, `editor/101–103` | `editor/103`, `editor/105` | `editor/87`, `editor/109` | apply certified weather degrade step and rerun visibility compare |
| cloth and fur | `editor/68`, `editor/75`, `editor/81` | `editor/68`, `editor/75`, `editor/103` | `editor/68`, `editor/100` | `editor/103`, `editor/105` | `editor/87`, `editor/109` | restore envelope baseline and rerun contact probes |
| population / tactics / ecology | `editor/69`, `editor/70`, `editor/71`, `editor/81` | `editor/69–71`, `editor/77`, `editor/103` | `editor/69–71`, `editor/100` | `editor/77`, `editor/103`, `editor/105` | `editor/87`, `editor/109` | restore horizon checkpoint or tactical / ecological baseline from the owning lab |
| wounds and semantic runtime | `editor/72`, `editor/73`, `editor/81` | `editor/72`, `editor/73`, `editor/80`, `editor/103` | `editor/72`, `editor/73`, `editor/100` | `editor/103`, `editor/105` | `editor/87`, `editor/109` | restore last-good trace or grounding bundle |
| presentation/runtime | `editor/90–98`, `editor/81` | `editor/90–98`, `editor/103`, `editor/105` | `editor/90–98`, `editor/100`, `editor/101–103` | `editor/103`, `editor/105` | `editor/87`, `editor/109` | restore the exact owning baseline and rerun the declared compare mode |
| brutal proof-region relay | `editor/82`, `editor/86`, `editor/103` | `editor/86`, `editor/103`, `editor/105` | `editor/86`, `editor/100`, `editor/109` | `editor/103`, `editor/105`, `editor/109` | `editor/87`, `editor/109` | restore retained proof-region baseline, then rerun compare and launch verification |
| combined old hardware / regression | `editor/81`, `editor/102`, `editor/103` | `editor/81`, `editor/102`, `editor/103`, `editor/105` | `editor/101–103`, `editor/109` | `editor/103`, `editor/105` | `editor/87`, `editor/109` | revert to combined baseline and apply certified ladders in reverse order |

## Mandatory fail card
Every certification failure must expose:
- pack id and scope;
- exact threshold row that failed;
- first failure family and exact failure code;
- current run id and last-good baseline id;
- packet manifest ids with compatibility posture;
- route attempt id with retry state;
- exact owning lab;
- exact next legal recovery action.

## Regression review law
- every regression row must point to one canonical `pack.*` id already registered in root `81`;
- regression review may aggregate many runs, but signoff must always resolve back to one retained baseline, one failed run, and one recovery run;
- no dashboard may invent a second certification verdict outside the retained pack rows;
- lifecycle regression remains blocking when route/packet lifecycle differs from the row declared in `editor/110`.

## Baseline protection law
- a red run may never replace the last-good baseline;
- an orange run may never replace the baseline unless the pack row explicitly marks the row as non-blocking and signoff records a waiver;
- a green run may replace the baseline only when the diff to the prior baseline is retained as its own artifact bundle;
- failed-run and recovery-run bundles must remain visibly distinct from baseline bundles.

## Freeze blocker law
Freeze remains blocked when any of the following is missing:
- retained baseline;
- failed-run artifact;
- recovery-run artifact when recovery was required;
- compare digest;
- hardware-floor result for freeze-relevant packs;
- blocker trace;
- first-result verification where a product relay is involved.

## Recovery scope guard
Recovery is legal only when it preserves the same truth owner family, packet family, route family, and lab surface declared by the failed pack row.
