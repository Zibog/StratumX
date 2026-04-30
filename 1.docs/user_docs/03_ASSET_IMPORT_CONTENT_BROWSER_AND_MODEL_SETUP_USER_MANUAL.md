# Asset Import, Content Browser, and Model Setup User Manual

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **user manual**.


## Import model

1. Open Content Browser.
2. Choose Import Asset.
3. Pick static mesh, skeletal mesh, terrain tile, foliage, groom, texture, material, audio, or animation family.
4. Review validation verdict.
5. Fix blockers.
6. Generate preview.
7. Bind materials/collision/LOD.
8. Cook package.
9. Place in world.

---

# V34 DCC import user workflow closure

Stack version: `SX-CANON/1.0.28/STACK-v34`

## User-facing rule

You may bring assets from Blender, 3ds Max, common exchange files, and supported add-on/plugin export paths. The editor must show whether the file was imported directly, through a DCC bridge, through a plugin adapter, through an external converter, or quarantined.

## Required import buttons

| Button | Result |
|---|---|
| Import File | Select `.blend`, `.max`, `.fbx`, `.glb`, `.gltf`, `.usd`, `.obj`, `.abc`, `.ply`, `.stl`, images, audio, and known source files. |
| Import From Blender | Uses configured Blender bridge. |
| Import From 3ds Max | Uses configured 3ds Max bridge. |
| Import Folder | Scans source tree, detects sidecars/textures/materials. |
| Reimport | Re-runs same route and preserves asset identity. |
| Show Conversion Report | Opens importer diagnostics. |
| Cook For Runtime | Produces `.sx*` runtime packages. |
| Open Blocker Help | Opens exact troubleshooting page. |

## Required user feedback

The content browser must show:

- source format;
- support class P0/P1/P2/P3/Q;
- importer route;
- converted/cooked asset ids;
- material conversion status;
- collision/LOD status;
- missing textures;
- unsupported plugin data;
- next legal action.

## User acceptance

A user can import a Blender asset and a 3ds Max asset, assign a StratumX material profile, place it in the world, see it in the viewport, cook it, save the project, close and reopen without losing identity.
