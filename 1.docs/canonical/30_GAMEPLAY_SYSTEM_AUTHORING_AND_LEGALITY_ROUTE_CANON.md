# Gameplay System Authoring And Legality Route Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

Gameplay authoring becomes lawful only when each authored change declares mutation class, allowed scope, route id, rollback anchor, and proof-region applicability.

| Gameplay family | Author / bind controls | Mutation class | Rollback anchor | Certification relevance |
|---|---|---|---|---|
| Need curves / faction rule | `btn.soc.author_need_curve`, `btn.soc.bind_faction_rule` | profile mutation | last-good social baseline | high |
| Squad doctrine / cover response | `btn.tac.author_doctrine`, `btn.tac.bind_cover_rule` | doctrine mutation | doctrine baseline | high |
| Species / migration / predation | `btn.eco.author_species_profile`, `btn.eco.bind_migration_corridor` | ecology mutation | ecology baseline | high |
| Anatomy / armor / wound rule | `btn.wound.author_anatomy_profile`, `btn.wound.bind_armor_stack` | anatomy mutation | wound baseline | high |
| Semantic grounding / world consequence | `btn.semantic.author_grounding_policy`, `btn.semantic.bind_world_consequence` | policy mutation | semantic baseline | high |
| Proof-region gameplay recipe | editor `108` gameplay recipe controls plus exact rows in `editor/110` | mixed gameplay mutation | proof-region recipe baseline | highest |

No gameplay authoring route may publish to runtime truth without validation, progress lifecycle, route-local artifact refs, and one restore-safe proof-region anchor.
