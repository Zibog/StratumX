use super::common::{string_args, suite_args, QualityContext};
use super::{smoke, verify};

pub fn run(ctx: &QualityContext) -> Result<(), String> {
    let mut stages = Vec::new();
    stages.push(
        ctx.record_stage("verify", "stratumx_quality_tasks verify", || {
            verify::run(ctx, false)
        })
        .inspect_err(|err| {
            let _ = ctx.write_task_summary("full", &stages, Some(err));
        })?,
    );
    stages.push(
        ctx.record_stage("smoke", "stratumx_quality_tasks smoke", || smoke::run(ctx))
            .inspect_err(|err| {
                let _ = ctx.write_task_summary("full", &stages, Some(err));
            })?,
    );

    let args = suite_args(&[
        "world_authoring_matrix",
        "terrain_authoring_matrix",
        "material_authoring_matrix",
        "environment_authoring_matrix",
        "audio_authoring_matrix",
        "build_release_matrix",
        "route_schema_golden",
        "focus_recovery_matrix",
        "proof_region_integration",
    ]);
    stages.push(
        ctx.run_cargo("full-matrix", &string_args(&args))
            .inspect_err(|err| {
                let _ = ctx.write_task_summary("full", &stages, Some(err));
            })?,
    );
    ctx.write_task_summary("full", &stages, None)
}
