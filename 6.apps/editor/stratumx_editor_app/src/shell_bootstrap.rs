// ShellBootstrap stub - editor initialization
use crate::editor_host::EditorHost;

pub struct ShellBootstrap;

impl ShellBootstrap {
    pub fn new() -> Self { Self }
    pub fn bootstrap(&mut self) -> Result<EditorHost, String> {
        let mut host = EditorHost::default();
        host.initialize()?;
        // startup() was removed during refactoring - world creation happens on demand
        Ok(host)
    }
}

impl Default for ShellBootstrap {
    fn default() -> Self { Self::new() }
}
