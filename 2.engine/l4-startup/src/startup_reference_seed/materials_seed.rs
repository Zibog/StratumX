// Materials seed - настройка material registry

use super::seed_ids::{SeedMaterialArchetype, SeedMaterialStack, SeedResponseProfile};
use engine_core::EngineCoreResult;
use engine_material::{
    ceramic_tile, concrete, plaster, MaterialLayer, MaterialRegistry, MaterialStack,
    MaterialStackId,
};

pub fn create_material_registry() -> EngineCoreResult<MaterialRegistry> {
    let fallback_profile_id = SeedResponseProfile::Default.canonical_id();
    let mut registry = MaterialRegistry::new(engine_material::MaterialConfig {
        fallback_descriptor: engine_material::MaterialDescriptor {
            material_id: engine_material::MaterialId(fallback_profile_id),
            label: "fallback".into(),
            property_domains: vec![engine_material::PropertyDomain::Physical],
            response_profile: engine_material::ResponseProfileId(fallback_profile_id),
        },
        default_reaction: engine_material::ReactionRow {
            response_profile: engine_material::ResponseProfileId(fallback_profile_id),
            coefficients: [1, 1, 1, 1],
        },
    });

    registry.register_archetype(ceramic_tile())?;
    registry.register_archetype(plaster())?;
    registry.register_archetype(concrete())?;

    Ok(registry)
}

pub fn create_wall_material_stack(
    materials: &mut MaterialRegistry,
) -> EngineCoreResult<MaterialStackId> {
    let stack = MaterialStack {
        id: MaterialStackId(SeedMaterialStack::WallTilePlasterConcrete.canonical_id()),
        label: "wall_tile_plaster_concrete".into(),
        layers: vec![
            MaterialLayer {
                archetype_id: engine_material::MaterialArchetypeId(
                    SeedMaterialArchetype::CeramicTile.canonical_id(),
                ),
                thickness_mm: 8.0,
                coverage: 1.0,
            },
            MaterialLayer {
                archetype_id: engine_material::MaterialArchetypeId(
                    SeedMaterialArchetype::Plaster.canonical_id(),
                ),
                thickness_mm: 15.0,
                coverage: 1.0,
            },
            MaterialLayer {
                archetype_id: engine_material::MaterialArchetypeId(
                    SeedMaterialArchetype::Concrete.canonical_id(),
                ),
                thickness_mm: 200.0,
                coverage: 1.0,
            },
        ],
    };

    materials.register_stack(stack)?;
    Ok(MaterialStackId(
        SeedMaterialStack::WallTilePlasterConcrete.canonical_id(),
    ))
}
