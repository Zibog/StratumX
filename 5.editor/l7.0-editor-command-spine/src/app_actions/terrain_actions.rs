// Terrain authoring actions

use super::ActionPayload;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

pub fn import_heightmap(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    payload
        .and_then(|p| p.as_string().map(|s| s.to_string()))
        .map(|heightmap_path| PromotedCommand::TerrainImport { heightmap_path })
}

pub fn rebuild(_payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    Some(PromotedCommand::TerrainRebuild)
}

pub fn sculpt_raise(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::TerrainOp {
        position,
        radius,
        strength,
    }) = payload
    {
        return Some(PromotedCommand::TerrainSculptRaise {
            position,
            radius,
            strength,
        });
    }
    None
}

pub fn sculpt_lower(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::TerrainOp {
        position,
        radius,
        strength,
    }) = payload
    {
        return Some(PromotedCommand::TerrainSculptLower {
            position,
            radius,
            strength,
        });
    }
    None
}

pub fn sculpt_smooth(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::TerrainOp {
        position,
        radius,
        strength,
    }) = payload
    {
        return Some(PromotedCommand::TerrainSculptSmooth {
            position,
            radius,
            strength,
        });
    }
    None
}

pub fn sculpt_flatten(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::TerrainOp {
        position,
        radius,
        strength,
    }) = payload
    {
        return Some(PromotedCommand::TerrainSculptFlatten {
            position,
            radius,
            strength,
            // Launch contour flattens toward the terrain datum plane until the
            // sampled target height is promoted into the action payload.
            target_height: 0.0,
        });
    }
    None
}

pub fn paint_material(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::TerrainOp {
        position,
        radius,
        strength,
    }) = payload
    {
        return Some(PromotedCommand::TerrainPaintMaterial {
            position,
            radius,
            strength,
            material_layer: 0,
        });
    }
    None
}

pub fn add_hole(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::TerrainOp {
        position, radius, ..
    }) = payload
    {
        return Some(PromotedCommand::TerrainAddHole { position, radius });
    }
    None
}

pub fn remove_hole(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::TerrainOp {
        position, radius, ..
    }) = payload
    {
        return Some(PromotedCommand::TerrainRemoveHole { position, radius });
    }
    None
}
