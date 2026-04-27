use crate::*;

impl EditorProduct {
    pub(crate) fn rebuild_from_document_objects(
        &mut self,
        objects: Vec<ToolObject>,
        selected_old: Option<ObjectHandle>,
    ) -> Result<(), String> {
        self.tooling = ToolingRuntime::new();
        self.world_suite.world_roots.clear();
        self.scene_suite.scene_entities.clear();
        self.terrain_suite.terrains.clear();
        self.material_suite.materials.clear();
        self.destruction_suite.fracture_targets.clear();
        self.simulation_suite.logic_nodes.clear();
        self.weather_suite.environment_nodes.clear();
        self.animation_suite.timelines.clear();
        self.audio_suite.audio_nodes.clear();
        self.ui_suite.widgets.clear();
        self.quest_suite.quest_nodes.clear();
        let mut remap = BTreeMap::new();
        for object in &objects {
            let new_handle = self
                .tooling
                .create_object(object.label.clone(), object.class)
                .map_err(|err| err.to_string())?;
            remap.insert(object.handle, new_handle);
            self.assign_suite_handle(new_handle, object.class);
        }
        for object in &objects {
            self.restore_object_payload(object, &remap)?;
        }
        if let Some(old_selected) = selected_old {
            if let Some(new_handle) = remap.get(&old_selected).copied() {
                self.viewport.selected = vec![new_handle];
                self.inspector.selected = Some(new_handle);
            }
        }
        self.refresh_from_tooling().map_err(|err| err.to_string())
    }
    fn assign_suite_handle(&mut self, handle: ObjectHandle, class: ObjectClass) {
        match class {
            ObjectClass::World => self.world_suite.world_roots.push(handle),
            ObjectClass::Scene => self.scene_suite.scene_entities.push(handle),
            ObjectClass::Terrain => self.terrain_suite.terrains.push(handle),
            ObjectClass::Material => self.material_suite.materials.push(handle),
            ObjectClass::Logic => {
                self.simulation_suite.logic_nodes.push(handle);
                self.quest_suite.quest_nodes.push(handle);
            }
            ObjectClass::Asset => self.ui_suite.widgets.push(handle),
            ObjectClass::Build => {}
        }
    }
    fn restore_object_payload(
        &mut self,
        object: &ToolObject,
        remap: &BTreeMap<ObjectHandle, ObjectHandle>,
    ) -> Result<(), String> {
        let new_handle = *remap.get(&object.handle).ok_or("object remap missing")?;
        for (key, value) in &object.fields {
            self.tooling
                .apply_command(
                    ToolCommand::UpsertField {
                        handle: new_handle,
                        key: key.clone(),
                        value: value.clone(),
                    },
                    CommandOrigin::User,
                    stratumx_tooling::ApprovalClass::None,
                    stratumx_tooling::BudgetClass::Interactive,
                )
                .map_err(|err| err.to_string())?;
        }
        for tag in &object.tags {
            self.tooling
                .apply_command(
                    ToolCommand::AddTag {
                        handle: new_handle,
                        tag: tag.clone(),
                    },
                    CommandOrigin::User,
                    stratumx_tooling::ApprovalClass::None,
                    stratumx_tooling::BudgetClass::Interactive,
                )
                .map_err(|err| err.to_string())?;
        }
        if !object.active {
            self.tooling
                .apply_command(
                    ToolCommand::RetireObject { handle: new_handle },
                    CommandOrigin::User,
                    stratumx_tooling::ApprovalClass::None,
                    stratumx_tooling::BudgetClass::Interactive,
                )
                .map_err(|err| err.to_string())?;
        }
        Ok(())
    }
}
