// Scene Command Handlers - Module Root

mod create;
mod entity_ops;
mod query;

use super::session::EditorAuthoringSession;
use link_egress_observations::{EditorAuthoringObservation, TransformDto};
use link_ingress_packets::SceneCommand;

fn transform_to_dto(t: link_ingress_packets::scene::Transform) -> TransformDto {
    TransformDto {
        position: t.position,
        rotation: t.rotation,
        scale: t.scale,
    }
}

pub fn handle(
    session: &mut EditorAuthoringSession,
    cmd: SceneCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        SceneCommand::CreateEmpty { scene_name } => {
            create::handle_create_empty(session, scene_name)
        }
        SceneCommand::CreateEntityFromAsset {
            asset_id,
            transform,
            label,
        } => create::handle_create_entity_from_asset(
            session,
            asset_id.to_string(),
            transform_to_dto(transform),
            label,
        ),
        SceneCommand::SetTransform {
            entity_id,
            transform,
        } => entity_ops::handle_set_transform(session, entity_id, transform_to_dto(transform)),
        SceneCommand::DeleteEntity { entity_id } => {
            entity_ops::handle_delete_entity(session, entity_id)
        }
        SceneCommand::ListEntities => query::handle_list_entities(session),
        SceneCommand::GetEntityDetails { entity_id } => {
            query::handle_get_entity_details(session, entity_id)
        }
    }
}
