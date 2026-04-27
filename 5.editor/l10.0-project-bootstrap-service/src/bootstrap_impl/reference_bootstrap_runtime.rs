use crate::*;

impl EditorProduct {
    pub fn new_reference(project_name: impl Into<String>) -> Result<Self, ToolingError> {
        let project_name = project_name.into();
        let mut tooling = ToolingRuntime::new();
        let world = tooling.create_object("world-root", ObjectClass::World)?;
        let scene = tooling.create_object("scene-root", ObjectClass::Scene)?;
        let material = tooling.create_object("material-root", ObjectClass::Material)?;
        let logic = tooling.create_object("logic-root", ObjectClass::Logic)?;
        Self::seed_reference_families(&mut tooling, world, scene, material, logic)?;
        let mut product =
            Self::reference_shell(project_name.clone(), tooling, world, scene, material, logic);
        product.refresh_from_tooling()?;
        Ok(product)
    }

    fn seed_reference_families(
        tooling: &mut ToolingRuntime,
        world: ObjectHandle,
        scene: ObjectHandle,
        material: ObjectHandle,
        logic: ObjectHandle,
    ) -> Result<(), ToolingError> {
        for (handle, value) in [
            (world, "world_scene_family"),
            (scene, "world_scene_family"),
            (material, "terrain_material_environment_family"),
            (logic, "quest_event_logic_family"),
        ] {
            tooling.apply_command(
                ToolCommand::UpsertField {
                    handle,
                    key: "family".into(),
                    value: value.into(),
                },
                CommandOrigin::User,
                stratumx_tooling::ApprovalClass::None,
                stratumx_tooling::BudgetClass::Interactive,
            )?;
        }
        tooling.apply_command(
            ToolCommand::AddTag {
                handle: scene,
                tag: "scene".into(),
            },
            CommandOrigin::User,
            stratumx_tooling::ApprovalClass::None,
            stratumx_tooling::BudgetClass::Interactive,
        )?;
        Ok(())
    }
}
