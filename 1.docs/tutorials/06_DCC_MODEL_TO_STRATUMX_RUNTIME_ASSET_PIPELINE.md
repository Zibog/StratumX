# Tutorial: DCC Model to StratumX Runtime Asset Pipeline

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **tutorial**.

## Goal

Import a Blender or 3ds Max source asset, convert it into StratumX cooked packages, assign a hybrid material profile, place it in the world, preview it, and capture evidence.

## Path A — Blender

1. Open Content Browser.
2. Click **Import From Blender**.
3. Select `.blend` or a Blender-exported `.fbx/.glb/.usd`.
4. Confirm Blender bridge path.
5. Run import.
6. Review conversion report:
   - objects;
   - units;
   - axes;
   - materials;
   - modifiers;
   - unsupported add-ons.
7. Assign or convert material to `opaque_pbr` or `terrain_pbr`.
8. Generate collision or choose no-collision with reason.
9. Cook to `.sxmesh`, `.sxmat`, `.sxtex`, optional `.sxcoll`.
10. Place asset in a world cell.
11. Open viewport.
12. Capture proof.

## Path B — 3ds Max

1. Open Content Browser.
2. Click **Import From 3ds Max**.
3. Select `.max` or exchange file.
4. Confirm 3ds Max bridge availability.
5. Choose export route: FBX, USD, glTF, OBJ, or adapter.
6. Review plugin/material conversion report.
7. Resolve unsupported V-Ray/Corona/Arnold/procedural/plugin rows.
8. Cook runtime packages.
9. Place in world.
10. Capture proof.

## Expected proof

- source descriptor;
- conversion report;
- cooked packages;
- material profile;
- viewport capture;
- evidence id.

## Failure variants

If bridge is missing, use exchange file. If plugin data is unsupported, bake/convert or quarantine. If material conversion fails, assign StratumX fallback profile and keep source map.
