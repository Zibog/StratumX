use crate::bridge_types::*;
use crate::tooling_runtime_core::ToolingRuntime;
use crate::tooling_types::*;
use std::borrow::Borrow;

impl ToolingRuntime {
    pub fn apply_command(
        &mut self,
        command: ToolCommand,
        origin: CommandOrigin,
        approval: ApprovalClass,
        budget: BudgetClass,
    ) -> Result<ToolCommandResult, String> {
        let result = match command.clone() {
            ToolCommand::CreateObject { label, class } => {
                let handle = self.create_object(label, class)?;
                ToolCommandResult::Created(handle)
            }
            ToolCommand::SetLabel { handle, label } => {
                let object = self
                    .bridge
                    .objects
                    .get_mut(&handle)
                    .ok_or("Object not found".to_string())?;
                object.label = label;
                ToolCommandResult::Modified
            }
            ToolCommand::UpsertField { handle, key, value } => {
                let object = self
                    .bridge
                    .objects
                    .get_mut(&handle)
                    .ok_or("Object not found".to_string())?;
                object.fields.insert(key, value);
                ToolCommandResult::Modified
            }
            ToolCommand::AddTag { handle, tag } => {
                let object = self
                    .bridge
                    .objects
                    .get_mut(&handle)
                    .ok_or("Object not found".to_string())?;
                object.tags.insert(tag);
                ToolCommandResult::Tagged
            }
            ToolCommand::SelectObject { handle } => {
                if self.bridge.object_view(handle).is_none() {
                    return Err("Object not found".to_string());
                }
                ToolCommandResult::Selected
            }
            ToolCommand::DeleteObject { handle } => {
                self.bridge
                    .objects
                    .remove(&handle)
                    .ok_or("Object not found".to_string())?;
                ToolCommandResult::Deleted
            }
        };

        self.generation = self.generation.saturating_add(1);
        self.ledger.push(ToolTransaction {
            command,
            result: result.clone(),
            order: self.next_order,
            origin,
            approval,
            budget,
            timestamp: self.bridge.next_epoch,
        });
        self.next_order = self.next_order.saturating_add(1);

        Ok(result)
    }

    pub fn validate_snapshot(&mut self) -> Vec<ValidationDiagnostic> {
        let mut diagnostics = Vec::new();

        if self.workspace.open_views.is_empty() || self.workspace.focused_view.trim().is_empty() {
            diagnostics.push(ValidationDiagnostic {
                severity: "error".to_string(),
                message: "workspace is empty".to_string(),
                handle: None,
            });
        }

        for object in self.bridge.objects.values() {
            if !object.fields.contains_key("family") {
                diagnostics.push(ValidationDiagnostic {
                    severity: "error".to_string(),
                    message: "missing required family field".to_string(),
                    handle: Some(object.handle),
                });
            }
            if object.label.trim().is_empty() {
                diagnostics.push(ValidationDiagnostic {
                    severity: "error".to_string(),
                    message: "label must not be empty".to_string(),
                    handle: Some(object.handle),
                });
            }
        }

        self.validation_history.push(diagnostics.clone());
        if self.validation_history.len() > 64 {
            let drain = self.validation_history.len() - 64;
            self.validation_history.drain(0..drain);
        }
        diagnostics
    }

    pub fn preview_object(&mut self, handle: ObjectHandle) -> Result<PreviewResult, String> {
        if self.bridge.object_view(handle).is_none() {
            return Err("Object not found".to_string());
        }

        let preview = PreviewResult {
            handle,
            preview_data: vec![0u8; 64],
            digest: format!("preview:{}:{}", self.generation, handle.opaque_tag()),
        };

        self.preview_cache.push(preview.clone());
        if self.preview_cache.len() > 32 {
            let drain = self.preview_cache.len() - 32;
            self.preview_cache.drain(0..drain);
        }
        Ok(preview)
    }

    pub fn build_current(&mut self) -> BuildArtifact {
        let object_count = self.bridge.objects.len();
        let invalidation_roots = if object_count == 0 {
            vec!["workspace".to_string()]
        } else {
            self.bridge
                .objects
                .keys()
                .map(|handle| handle.opaque_tag())
                .collect()
        };
        let digest = format!("build:{}:{}", self.generation, object_count);
        BuildArtifact {
            manifest: BuildManifest {
                kind: ArtifactClass::Build,
                digest: digest.clone(),
                invalidation_roots,
                artifact_count: object_count,
                total_size_bytes: 1024,
            },
            object_count,
            digest,
        }
    }

    pub fn release_build<T: Borrow<BuildArtifact>>(
        &mut self,
        build: T,
        channel: impl Into<String>,
        _signed: bool,
    ) -> Result<ReleasePackage, String> {
        let build = build.borrow();
        Ok(ReleasePackage {
            manifest: ReleaseManifest {
                channel: channel.into(),
                digest: format!("release:{}", build.digest),
            },
            build_digest: build.digest.clone(),
        })
    }

    pub fn stage_proposal(&mut self, goal: impl Into<String>, commands: Vec<ToolCommand>) -> u64 {
        let id = self.next_proposal_id;
        self.next_proposal_id += 1;

        self.proposals.push(AssistantProposal {
            id,
            goal: goal.into(),
            commands,
            approved: false,
        });

        id
    }

    pub fn approve_proposal(&mut self, proposal_id: u64) -> Result<(), String> {
        if let Some(proposal) = self.proposals.iter_mut().find(|p| p.id == proposal_id) {
            proposal.approved = true;
            Ok(())
        } else {
            Err("Proposal not found".to_string())
        }
    }

    pub fn apply_proposal(&mut self, proposal_id: u64) -> Result<Vec<ToolCommandResult>, String> {
        let proposal = self
            .proposals
            .iter()
            .find(|p| p.id == proposal_id)
            .ok_or("Proposal not found")?;

        if !proposal.approved {
            return Err("Proposal not approved".to_string());
        }

        let commands = proposal.commands.clone();
        self.assistant_evidence.push(AssistantEvidence {
            proposal_id,
            goal: proposal.goal.clone(),
            commands: commands.clone(),
        });
        let mut results = Vec::new();

        for command in commands {
            let result = self.apply_command(
                command,
                CommandOrigin::Assistant,
                ApprovalClass::None,
                BudgetClass::Background,
            )?;
            results.push(result);
        }

        Ok(results)
    }

    pub fn revert_proposal(&mut self, proposal_id: u64) -> Result<Vec<ToolCommand>, String> {
        let proposal = self
            .proposals
            .iter()
            .find(|p| p.id == proposal_id)
            .ok_or("Proposal not found")?;

        Ok(proposal.commands.clone())
    }
}
