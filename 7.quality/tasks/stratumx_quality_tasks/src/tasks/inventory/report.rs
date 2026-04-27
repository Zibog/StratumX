use super::super::common::QualityContext;
use std::fs;
use std::path::PathBuf;

pub fn ensure_docs_dir(ctx: &QualityContext) -> Result<PathBuf, String> {
    let docs_dir = ctx.repo_root.join("7.quality").join("docs");
    fs::create_dir_all(&docs_dir).map_err(|e| format!("Failed to create docs directory: {}", e))?;
    Ok(docs_dir)
}
