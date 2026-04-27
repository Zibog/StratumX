// Material Command Handlers

use super::session::EditorAuthoringSession;
use link_egress_observations::*;
use link_ingress_packets::MaterialCommand;

pub fn handle(
    session: &mut EditorAuthoringSession,
    cmd: MaterialCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        MaterialCommand::CreateArchetype { input } => {
            let archetype_id = session.next_archetype_id;
            session.next_archetype_id += 1;

            let archetype = MaterialArchetypeDto {
                archetype_id,
                label: input.label,
                mechanical_class: input.mechanical_class,
                hardness_mohs: input.hardness_mohs,
                brittleness: input.brittleness,
                density_kg_m3: input.density_kg_m3,
                fracture_mode: input.fracture_mode,
                penetration_resistance: input.penetration_resistance,
                ricochet_bias: input.ricochet_bias,
                debris_profile: input.debris_profile,
                dust_amount: input.dust_amount,
            };

            session.archetypes.insert(archetype_id, archetype.clone());
            Ok(EditorAuthoringObservation::MaterialArchetypeCreated { archetype })
        }
        MaterialCommand::ListArchetypes => {
            let archetypes: Vec<MaterialArchetypeDto> =
                session.archetypes.values().cloned().collect();
            Ok(EditorAuthoringObservation::MaterialArchetypeList { archetypes })
        }
        MaterialCommand::CreateStack { label } => {
            let stack_id = session.next_stack_id;
            session.next_stack_id += 1;

            let stack = AuthoringMaterialStackDto {
                stack_id,
                label,
                layers: vec![],
                total_thickness_mm: 0.0,
            };

            session.stacks.insert(stack_id, stack.clone());
            Ok(EditorAuthoringObservation::MaterialStackCreated { stack })
        }
        MaterialCommand::StackAddLayer { stack_id, layer } => {
            if let Some(stack) = session.stacks.get_mut(&stack_id) {
                let archetype_label = session
                    .archetypes
                    .get(&layer.archetype_id)
                    .map(|a| a.label.clone())
                    .unwrap_or_else(|| format!("archetype_{}", layer.archetype_id));

                stack.layers.push(AuthoringMaterialLayerDto {
                    archetype_id: layer.archetype_id,
                    archetype_label,
                    thickness_mm: layer.thickness_mm,
                    coverage: layer.coverage,
                    bond_strength: layer.bond_strength,
                    segmentation_mode: layer.segmentation_mode,
                    segment_size_mm: layer.segment_size_mm,
                });
                stack.total_thickness_mm = stack.layers.iter().map(|l| l.thickness_mm).sum();

                Ok(EditorAuthoringObservation::MaterialStackUpdated {
                    stack: stack.clone(),
                })
            } else {
                Err(format!("Stack {} not found", stack_id))
            }
        }
        MaterialCommand::StackRemoveLayer {
            stack_id,
            layer_index,
        } => {
            if let Some(stack) = session.stacks.get_mut(&stack_id) {
                if (layer_index as usize) < stack.layers.len() {
                    stack.layers.remove(layer_index as usize);
                    stack.total_thickness_mm = stack.layers.iter().map(|l| l.thickness_mm).sum();
                    Ok(EditorAuthoringObservation::MaterialStackUpdated {
                        stack: stack.clone(),
                    })
                } else {
                    Err(format!("Layer index {} out of bounds", layer_index))
                }
            } else {
                Err(format!("Stack {} not found", stack_id))
            }
        }
        MaterialCommand::AssignStackToEntitySlot {
            entity_id,
            slot_id,
            stack_id,
        } => {
            if let Some(vs) = &mut session.vertical_slice_session {
                if entity_id == 2 {
                    vs.authoring_update_wall_material(stack_id)?;
                    Ok(EditorAuthoringObservation::MaterialAssigned {
                        entity_id,
                        slot_id,
                        stack_id,
                    })
                } else {
                    Err("Material assignment only supported for wall (entity_id=2)".into())
                }
            } else {
                Err("Runtime session not initialized".into())
            }
        }
        MaterialCommand::GetStack { stack_id } => {
            if let Some(stack) = session.stacks.get(&stack_id) {
                Ok(EditorAuthoringObservation::MaterialStackDetails {
                    stack: stack.clone(),
                })
            } else {
                Err(format!("Stack {} not found", stack_id))
            }
        }
        MaterialCommand::ListStacks => {
            let stacks: Vec<AuthoringMaterialStackDto> = session.stacks.values().cloned().collect();
            Ok(EditorAuthoringObservation::MaterialStackList { stacks })
        }
    }
}
