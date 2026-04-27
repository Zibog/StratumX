use crate::*;

type TemplateFields = Vec<(&'static str, &'static str)>;
type TemplateTags = Vec<&'static str>;
type TemplateObject = (String, ObjectClass, TemplateFields, TemplateTags);

impl EditorProduct {
    pub fn create_template(
        &mut self,
        template: EditorTemplateKind,
        stem: impl Into<String>,
    ) -> Result<Vec<ObjectHandle>, ToolingError> {
        let stem = stem.into();
        let mut handles = Vec::new();
        for (label, class, fields, tags) in self.template_objects(template, &stem) {
            let handle = self.create_object_with_class(label, class)?;
            self.apply_template_payload(handle, fields, tags)?;
            handles.push(handle);
        }
        self.refresh_from_tooling()?;
        Ok(handles)
    }
    fn apply_template_payload(
        &mut self,
        handle: ObjectHandle,
        fields: TemplateFields,
        tags: TemplateTags,
    ) -> Result<(), ToolingError> {
        for (key, value) in fields {
            self.tooling.apply_command(
                ToolCommand::UpsertField {
                    handle,
                    key: key.into(),
                    value: value.into(),
                },
                CommandOrigin::User,
                stratumx_tooling::ApprovalClass::None,
                stratumx_tooling::BudgetClass::Interactive,
            )?;
        }
        for tag in tags {
            self.tooling.apply_command(
                ToolCommand::AddTag {
                    handle,
                    tag: tag.into(),
                },
                CommandOrigin::User,
                stratumx_tooling::ApprovalClass::None,
                stratumx_tooling::BudgetClass::Interactive,
            )?;
        }
        Ok(())
    }
    fn template_objects(&self, template: EditorTemplateKind, stem: &str) -> Vec<TemplateObject> {
        match template {
            EditorTemplateKind::OpenWorldScaffold => vec![
                (
                    format!("{stem}-world"),
                    ObjectClass::World,
                    vec![("family", "world_scene_family"), ("role", "world_root")],
                    vec!["world", "streaming"],
                ),
                (
                    format!("{stem}-scene"),
                    ObjectClass::Scene,
                    vec![("family", "world_scene_family"), ("role", "play_space")],
                    vec!["scene", "spawn"],
                ),
                (
                    format!("{stem}-logic"),
                    ObjectClass::Logic,
                    vec![
                        ("family", "quest_event_logic_family"),
                        ("role", "bootstrap"),
                    ],
                    vec!["logic", "bootstrap"],
                ),
            ],
            EditorTemplateKind::TerrainMaterialStack => vec![
                (
                    format!("{stem}-terrain"),
                    ObjectClass::Terrain,
                    vec![
                        ("family", "terrain_material_environment_family"),
                        ("role", "landscape"),
                    ],
                    vec!["terrain", "paintable"],
                ),
                (
                    format!("{stem}-material"),
                    ObjectClass::Material,
                    vec![
                        ("family", "terrain_material_environment_family"),
                        ("role", "master_material"),
                    ],
                    vec!["material", "layered"],
                ),
            ],
            EditorTemplateKind::QuestGameplaySlice => vec![
                (
                    format!("{stem}-quest"),
                    ObjectClass::Logic,
                    vec![("family", "quest_event_logic_family"), ("role", "quest")],
                    vec!["quest", "gameplay"],
                ),
                (
                    format!("{stem}-trigger"),
                    ObjectClass::Scene,
                    vec![("family", "world_scene_family"), ("role", "trigger")],
                    vec!["scene", "trigger"],
                ),
            ],
            EditorTemplateKind::WeatherEnvironmentProfile => vec![
                (
                    format!("{stem}-weather"),
                    ObjectClass::Logic,
                    vec![
                        ("family", "terrain_material_environment_family"),
                        ("role", "weather_profile"),
                    ],
                    vec!["weather", "environment"],
                ),
                (
                    format!("{stem}-sky"),
                    ObjectClass::Asset,
                    vec![
                        ("family", "terrain_material_environment_family"),
                        ("role", "sky_asset"),
                    ],
                    vec!["asset", "sky"],
                ),
            ],
            EditorTemplateKind::AudioUiPack => vec![
                (
                    format!("{stem}-audio"),
                    ObjectClass::Asset,
                    vec![("family", "audio_voice_family"), ("role", "audio_bank")],
                    vec!["audio", "voice"],
                ),
                (
                    format!("{stem}-hud"),
                    ObjectClass::Asset,
                    vec![("family", "ui_hud_family"), ("role", "hud")],
                    vec!["ui", "hud"],
                ),
            ],
        }
    }
}
