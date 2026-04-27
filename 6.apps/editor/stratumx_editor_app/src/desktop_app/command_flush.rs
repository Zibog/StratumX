// Command flush module - thin shim
// Per canon: command flushing logic moved to 5.editor/l8.0-editor-shell

impl super::EditorApp {
    pub fn flush_shell_commands_to_host(&mut self) {
        // Thin host: no complex command flushing
        // All commands are processed immediately through action adapters
    }
}
