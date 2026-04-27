//! Unit tests for Command Spine
//!
//! These tests verify specific examples and edge cases.

use stratumx_editor_l7_0_editor_command_spine::{
    Command, CommandSpine, CommandType,
};

// ============================================================================
// Tests for invalid commands
// ============================================================================

#[test]
fn test_empty_custom_command_rejected() {
    let spine = CommandSpine::new();
    let command = Command::new(CommandType::Custom("".to_string()));
    
    let result = spine.validate_command(&command);
    assert!(result.is_err());
    assert!(result.unwrap_err().reason.contains("empty"));
}

#[test]
fn test_valid_custom_command_accepted() {
    let spine = CommandSpine::new();
    let command = Command::new(CommandType::Custom("my_command".to_string()));
    
    let result = spine.validate_command(&command);
    assert!(result.is_ok());
}

#[test]
fn test_builtin_commands_always_valid() {
    let spine = CommandSpine::new();
    
    let commands = vec![
        CommandType::SculptTerrain,
        CommandType::PaintTerrain,
        CommandType::ImportHeightmap,
        CommandType::CreateMaterial,
        CommandType::AssignTexture,
        CommandType::SelectEntity,
        CommandType::DeselectAll,
        CommandType::TranslateEntity,
        CommandType::RotateEntity,
        CommandType::ScaleEntity,
        CommandType::SaveProject,
        CommandType::LoadProject,
    ];
    
    for cmd_type in commands {
        let command = Command::new(cmd_type);
        assert!(spine.validate_command(&command).is_ok());
    }
}

// ============================================================================
// Tests for undo/redo sequences
// ============================================================================

#[test]
fn test_undo_redo_sequence() {
    let mut spine = CommandSpine::with_state(true, true, true);
    
    // Execute three commands
    let cmd1 = Command::new(CommandType::SelectEntity);
    let cmd2 = Command::new(CommandType::TranslateEntity);
    let cmd3 = Command::new(CommandType::RotateEntity);
    
    spine.execute_command(cmd1).unwrap();
    spine.execute_command(cmd2).unwrap();
    spine.execute_command(cmd3).unwrap();
    
    assert_eq!(spine.undo_count(), 3);
    assert_eq!(spine.redo_count(), 0);
    
    // Undo twice
    spine.undo().unwrap();
    spine.undo().unwrap();
    
    assert_eq!(spine.undo_count(), 1);
    assert_eq!(spine.redo_count(), 2);
    
    // Redo once
    spine.redo().unwrap();
    
    assert_eq!(spine.undo_count(), 2);
    assert_eq!(spine.redo_count(), 1);
}

#[test]
fn test_new_command_clears_redo_stack() {
    let mut spine = CommandSpine::with_state(true, true, true);
    
    // Execute two commands
    let cmd1 = Command::new(CommandType::SelectEntity);
    let cmd2 = Command::new(CommandType::TranslateEntity);
    
    spine.execute_command(cmd1).unwrap();
    spine.execute_command(cmd2).unwrap();
    
    // Undo one
    spine.undo().unwrap();
    assert_eq!(spine.redo_count(), 1);
    assert_eq!(spine.undo_count(), 1);
    
    // Execute new command
    let cmd3 = Command::new(CommandType::RotateEntity);
    spine.execute_command(cmd3).unwrap();
    
    // Redo stack should be cleared
    assert_eq!(spine.redo_count(), 0);
    // Undo stack should have the first command + new command
    assert_eq!(spine.undo_count(), 2);
}

#[test]
fn test_undo_all_then_redo_all() {
    let mut spine = CommandSpine::with_state(true, true, true);
    
    // Execute three commands
    spine.execute_command(Command::new(CommandType::SelectEntity)).unwrap();
    spine.execute_command(Command::new(CommandType::TranslateEntity)).unwrap();
    spine.execute_command(Command::new(CommandType::RotateEntity)).unwrap();
    
    let initial_undo_count = spine.undo_count();
    
    // Undo all
    while spine.undo_count() > 0 {
        spine.undo().unwrap();
    }
    
    assert_eq!(spine.undo_count(), 0);
    assert_eq!(spine.redo_count(), initial_undo_count);
    
    // Redo all
    while spine.redo_count() > 0 {
        spine.redo().unwrap();
    }
    
    assert_eq!(spine.undo_count(), initial_undo_count);
    assert_eq!(spine.redo_count(), 0);
}

// ============================================================================
// Tests for edge cases
// ============================================================================

#[test]
fn test_undo_on_empty_stack() {
    let mut spine = CommandSpine::new();
    
    let result = spine.undo();
    assert!(result.is_err());
    assert!(result.unwrap_err().reason.contains("No commands to undo"));
}

#[test]
fn test_redo_on_empty_stack() {
    let mut spine = CommandSpine::new();
    
    let result = spine.redo();
    assert!(result.is_err());
    assert!(result.unwrap_err().reason.contains("No commands to redo"));
}

#[test]
fn test_command_history_tracks_all_commands() {
    let mut spine = CommandSpine::with_state(true, true, true);
    
    assert_eq!(spine.command_history().len(), 0);
    
    spine.execute_command(Command::new(CommandType::SelectEntity)).unwrap();
    assert_eq!(spine.command_history().len(), 1);
    
    spine.execute_command(Command::new(CommandType::TranslateEntity)).unwrap();
    assert_eq!(spine.command_history().len(), 2);
    
    spine.execute_command(Command::new(CommandType::RotateEntity)).unwrap();
    assert_eq!(spine.command_history().len(), 3);
}

#[test]
fn test_failed_command_not_in_history() {
    let mut spine = CommandSpine::new(); // No project, no world, no selection
    
    // This command should fail preconditions
    let command = Command::new(CommandType::TranslateEntity);
    let result = spine.execute_command(command);
    
    assert!(result.is_err());
    assert_eq!(spine.command_history().len(), 0);
}

#[test]
fn test_preconditions_project_required() {
    let spine = CommandSpine::new(); // No project
    
    let commands = vec![
        CommandType::SaveProject,
        CommandType::LoadProject,
        CommandType::CreateMaterial,
        CommandType::AssignTexture,
        CommandType::SculptTerrain,
        CommandType::PaintTerrain,
        CommandType::ImportHeightmap,
    ];
    
    for cmd_type in commands {
        let command = Command::new(cmd_type);
        let result = spine.check_preconditions(&command);
        assert!(result.is_err());
        assert!(result.unwrap_err().failed_preconditions.contains(&"No project loaded".to_string()));
    }
}

#[test]
fn test_preconditions_world_required() {
    let spine = CommandSpine::with_state(true, false, false); // Project but no world
    
    let commands = vec![
        CommandType::SculptTerrain,
        CommandType::PaintTerrain,
        CommandType::ImportHeightmap,
    ];
    
    for cmd_type in commands {
        let command = Command::new(cmd_type);
        let result = spine.check_preconditions(&command);
        assert!(result.is_err());
        assert!(result.unwrap_err().failed_preconditions.contains(&"No active world".to_string()));
    }
}

#[test]
fn test_preconditions_selection_required() {
    let spine = CommandSpine::with_state(true, true, false); // Project and world but no selection
    
    let commands = vec![
        CommandType::TranslateEntity,
        CommandType::RotateEntity,
        CommandType::ScaleEntity,
    ];
    
    for cmd_type in commands {
        let command = Command::new(cmd_type);
        let result = spine.check_preconditions(&command);
        assert!(result.is_err());
        assert!(result.unwrap_err().failed_preconditions.contains(&"No entities selected".to_string()));
    }
}

#[test]
fn test_action_id_consistency() {
    let spine = CommandSpine::new();
    
    let cmd1 = Command::new(CommandType::SculptTerrain);
    let cmd2 = Command::new(CommandType::SculptTerrain);
    
    let action_id1 = spine.get_action_id(&cmd1);
    let action_id2 = spine.get_action_id(&cmd2);
    
    // Same command type should produce same action ID
    assert_eq!(action_id1, action_id2);
}

#[test]
fn test_action_id_uniqueness() {
    let spine = CommandSpine::new();
    
    let cmd1 = Command::new(CommandType::SculptTerrain);
    let cmd2 = Command::new(CommandType::PaintTerrain);
    
    let action_id1 = spine.get_action_id(&cmd1);
    let action_id2 = spine.get_action_id(&cmd2);
    
    // Different command types should produce different action IDs
    assert_ne!(action_id1, action_id2);
}

#[test]
fn test_custom_command_action_ids() {
    let spine = CommandSpine::new();
    
    let cmd1 = Command::new(CommandType::Custom("foo".to_string()));
    let cmd2 = Command::new(CommandType::Custom("bar".to_string()));
    let cmd3 = Command::new(CommandType::Custom("foo".to_string()));
    
    let action_id1 = spine.get_action_id(&cmd1);
    let action_id2 = spine.get_action_id(&cmd2);
    let action_id3 = spine.get_action_id(&cmd3);
    
    // Same custom command name should produce same action ID
    assert_eq!(action_id1, action_id3);
    
    // Different custom command names should produce different action IDs
    assert_ne!(action_id1, action_id2);
}
