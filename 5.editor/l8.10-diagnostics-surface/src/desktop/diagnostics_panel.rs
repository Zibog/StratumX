//! Diagnostics panel trimmed to real launch data.

use eframe::egui;

use super::diagnostics_state::{DiagnosticsState, DiagnosticsTab, RecoveryAnchor};

/// Renders the diagnostics panel.
/// This function is a template for integration into an EditorApp or similar UI type.
pub fn render_diagnostics_panel(
    ui: &mut egui::Ui,
    show_diagnostics: bool,
    diagnostics_tab: &mut DiagnosticsTab,
    diagnostics_state: &DiagnosticsState,
    render_system_fn: impl FnOnce(&mut egui::Ui),
) {
    if !show_diagnostics {
        return;
    }

    ui.horizontal(|ui| {
        ui.heading("Diagnostics");
        ui.selectable_value(diagnostics_tab, DiagnosticsTab::System, "System");
        ui.selectable_value(diagnostics_tab, DiagnosticsTab::Coverage, "Coverage");
        ui.selectable_value(diagnostics_tab, DiagnosticsTab::Traces, "Traces");
        ui.selectable_value(diagnostics_tab, DiagnosticsTab::Proof, "Proof");
    });

    ui.separator();

    match diagnostics_tab {
        DiagnosticsTab::System => render_system_fn(ui),
        DiagnosticsTab::Coverage => render_coverage_diagnostics(ui, diagnostics_state),
        DiagnosticsTab::Traces => render_trace_diagnostics(ui, diagnostics_state),
        DiagnosticsTab::Proof => render_proof_rail(ui, diagnostics_state),
    }
}

fn render_coverage_diagnostics(ui: &mut egui::Ui, state: &DiagnosticsState) {
    ui.add(egui::ProgressBar::new(state.runtime_coverage).text(format!(
        "{:.0}% runtime coverage",
        state.runtime_coverage * 100.0
    )));
    ui.add(egui::ProgressBar::new(state.visual_coverage).text(format!(
        "{:.0}% visual coverage",
        state.visual_coverage * 100.0
    )));

    if state.missing_coverage.is_empty() {
        ui.label("No recorded coverage gaps");
    } else {
        for missing in &state.missing_coverage {
            ui.label(format!("{}: {}", missing.branch, missing.missing_bindings));
        }
    }
}

fn render_trace_diagnostics(ui: &mut egui::Ui, state: &DiagnosticsState) {
    if state.trace_chains.is_empty() {
        ui.label("No trace chains recorded");
        return;
    }

    for trace in &state.trace_chains {
        ui.collapsing(trace.route_id.clone(), |ui| {
            ui.label(format!("Result: {}", trace.result));
            if !trace.denial_family.is_empty() {
                ui.label(format!("Denial: {}", trace.denial_family));
            }
            if !trace.recovery_target.is_empty() {
                ui.label(format!("Recovery: {}", trace.recovery_target));
            }
        });
    }
}

fn render_proof_rail(ui: &mut egui::Ui, state: &DiagnosticsState) {
    ui.label(format!(
        "Freeze Ready: {}",
        if state.freeze_ready { "yes" } else { "no" }
    ));

    for blocker in &state.proof_blockers {
        ui.label(format!("Blocker: {}", blocker.description));
    }
}

/// Creates a new recovery anchor.
pub fn create_recovery_anchor(state: &mut DiagnosticsState) -> RecoveryAnchor {
    let index = state.recovery_anchors.len() + 1;
    RecoveryAnchor {
        id: format!("anchor_{}", index),
        label: format!("State {}", index),
        timestamp: format!("{:?}", std::time::SystemTime::now()),
        snapshot: Default::default(),
    }
}
