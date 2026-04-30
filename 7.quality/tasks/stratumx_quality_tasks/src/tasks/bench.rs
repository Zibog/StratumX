use super::common::QualityContext;

pub fn run(ctx: &QualityContext) -> Result<(), String> {
    let record = ctx
        .run_cargo("bench", &["bench", "-p", "engine_perf_harness"])
        .inspect_err(|err| {
            let _ = ctx.write_task_summary("bench", &[], Some(err));
        })?;
    ctx.write_task_summary("bench", &[record], None)
}
