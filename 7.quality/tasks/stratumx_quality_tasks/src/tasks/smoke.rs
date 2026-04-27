use super::common::{string_args, suite_args, QualityContext};

pub fn run(ctx: &QualityContext) -> Result<(), String> {
    let args = suite_args(&["smoke"]);
    let record = ctx.run_cargo("smoke", &string_args(&args)).map_err(|err| {
        let _ = ctx.write_task_summary("smoke", &[], Some(&err));
        err
    })?;
    ctx.write_task_summary("smoke", &[record], None)
}
