use crate::*;

impl EditorProduct {
    pub fn create_object_with_class(
        &mut self,
        label: impl Into<String>,
        class: ObjectClass,
    ) -> Result<ObjectHandle, ToolingError> {
        let handle = self.tooling.create_object(label.into(), class)?;
        if let Some(family) = self.default_family_for(class) {
            self.write_family_field(handle, family)?;
        }
        if let Some(tag) = self.default_tag_for(class) {
            self.write_default_tag(handle, tag)?;
        }
        self.assign_created_object(handle, class);
        self.select_object(handle)?;
        self.refresh_from_tooling()?;
        Ok(handle)
    }
    fn default_family_for(&self, class: ObjectClass) -> Option<&'static str> {
        match class {
            ObjectClass::World | ObjectClass::Scene => Some("world_scene_family"),
            ObjectClass::Terrain | ObjectClass::Material | ObjectClass::Asset => {
                Some("terrain_material_environment_family")
            }
            ObjectClass::Logic => Some("quest_event_logic_family"),
            ObjectClass::Build => Some("build_validation_release_family"),
        }
    }
    fn default_tag_for(&self, class: ObjectClass) -> Option<&'static str> {
        match class {
            ObjectClass::World => Some("world"),
            ObjectClass::Scene => Some("scene"),
            ObjectClass::Terrain => Some("terrain"),
            ObjectClass::Material => Some("material"),
            ObjectClass::Logic => Some("logic"),
            ObjectClass::Asset => Some("asset"),
            ObjectClass::Build => Some("build"),
        }
    }
    fn write_family_field(
        &mut self,
        handle: ObjectHandle,
        family: &str,
    ) -> Result<(), ToolingError> {
        self.tooling.apply_command(
            ToolCommand::UpsertField {
                handle,
                key: "family".into(),
                value: family.into(),
            },
            CommandOrigin::User,
            stratumx_tooling::ApprovalClass::None,
            stratumx_tooling::BudgetClass::Interactive,
        )?;
        Ok(())
    }
    fn write_default_tag(&mut self, handle: ObjectHandle, tag: &str) -> Result<(), ToolingError> {
        self.tooling.apply_command(
            ToolCommand::AddTag {
                handle,
                tag: tag.into(),
            },
            CommandOrigin::User,
            stratumx_tooling::ApprovalClass::None,
            stratumx_tooling::BudgetClass::Interactive,
        )?;
        Ok(())
    }
    fn assign_created_object(&mut self, handle: ObjectHandle, class: ObjectClass) {
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
}
