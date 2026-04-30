//! Property-based tests for Command Spine
//!
//! These tests verify universal properties that should hold for all valid inputs.

use proptest::prelude::*;
use stratumx_editor_l7_0_editor_command_spine::{
    Command, CommandParameters, CommandSpine, CommandType,
};

// ============================================================================
// Generators for property-based testing
// ============================================================================

/// Generates arbitrary CommandType
fn arbitrary_command_type() -> impl Strategy<Value = CommandType> {
    prop_oneof![
        Just(CommandType::SculptTerrain),
        Just(CommandType::PaintTerrain),
        Just(CommandType::ImportHeightmap),
        Just(CommandType::CreateMaterial),
        Just(CommandType::AssignTexture),
        Just(CommandType::SelectEntity),
        Just(CommandType::DeselectAll),
        Just(CommandType::TranslateEntity),
        Just(CommandType::RotateEntity),
        Just(CommandType::ScaleEntity),
        Just(CommandType::SaveProject),
        Just(CommandType::LoadProject),
        "[a-z]{1,20}".prop_map(CommandType::Custom),
    ]
}

/// Generates arbitrary CommandParameters
fn arbitrary_command_parameters() -> impl Strategy<Value = CommandParameters> {
    prop::collection::hash_map("[a-z]{1,10}", "[a-z0-9]{0,20}", 0..5).prop_map(|params| {
        let mut cmd_params = CommandParameters::new();
        for (key, value) in params {
            cmd_params = cmd_params.with_param(key, value);
        }
        cmd_params
    })
}

/// Generates arbitrary Command
fn arbitrary_command() -> impl Strategy<Value = Command> {
    (arbitrary_command_type(), arbitrary_command_parameters())
        .prop_map(|(cmd_type, params)| Command::with_parameters(cmd_type, params))
}

// ============================================================================
// Property 3: Валидация команд перед выполнением
// ============================================================================

// Feature: editor-apps-proof-lane-closure, Property 3: Валидация команд перед выполнением
// **Validates: Requirements 2.1**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_commands_validated_before_execution(command in arbitrary_command()) {
        let mut spine = CommandSpine::new();

        // Execute the command
        let result = spine.dispatch_command(command.clone());

        // If command was executed successfully, it must have passed validation
        if result.is_ok() {
            prop_assert!(spine.validate_command(&command).is_ok());
        }

        // If validation fails, execution must also fail
        if spine.validate_command(&command).is_err() {
            prop_assert!(result.is_err());
        }
    }
}

// Feature: editor-apps-proof-lane-closure, Property 3: Валидация команд перед выполнением
// **Validates: Requirements 2.1**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_invalid_commands_rejected(custom_name in ".*") {
        let spine = CommandSpine::new();
        let command = Command::new(CommandType::Custom(custom_name.clone()));

        let validation_result = spine.validate_command(&command);

        // Empty custom command names should be rejected
        if custom_name.is_empty() {
            prop_assert!(validation_result.is_err());
        } else {
            prop_assert!(validation_result.is_ok());
        }
    }
}

// Feature: editor-apps-proof-lane-closure, Property 3: Валидация команд перед выполнением
// **Validates: Requirements 2.1**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_builtin_commands_always_valid(
        cmd_type in prop_oneof![
            Just(CommandType::SculptTerrain),
            Just(CommandType::PaintTerrain),
            Just(CommandType::ImportHeightmap),
            Just(CommandType::CreateMaterial),
            Just(CommandType::AssignTexture),
            Just(CommandType::SelectEntity),
            Just(CommandType::DeselectAll),
            Just(CommandType::TranslateEntity),
            Just(CommandType::RotateEntity),
            Just(CommandType::ScaleEntity),
            Just(CommandType::SaveProject),
            Just(CommandType::LoadProject),
        ]
    ) {
        let spine = CommandSpine::new();
        let command = Command::new(cmd_type);

        // All built-in command types should always be valid
        prop_assert!(spine.validate_command(&command).is_ok());
    }
}

// ============================================================================
// Property 6: Идентификаторы действий для команд
// ============================================================================

// Feature: editor-apps-proof-lane-closure, Property 6: Идентификаторы действий для команд
// **Validates: Requirements 2.4**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_action_ids_are_unique_per_command_type(command in arbitrary_command()) {
        let spine = CommandSpine::new();

        // Get action ID for the command
        let action_id = spine.get_action_id(&command);

        // Action ID should be non-zero
        prop_assert_ne!(action_id.as_u64(), 0);

        // Create another command of the same type
        let command2 = Command::new(command.command_type.clone());
        let action_id2 = spine.get_action_id(&command2);

        // Same command type should produce same action ID
        prop_assert_eq!(action_id, action_id2);
    }
}

// Feature: editor-apps-proof-lane-closure, Property 6: Идентификаторы действий для команд
// **Validates: Requirements 2.4**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_action_ids_deterministic(command in arbitrary_command()) {
        let spine = CommandSpine::new();

        // Get action ID multiple times
        let action_id1 = spine.get_action_id(&command);
        let action_id2 = spine.get_action_id(&command);
        let action_id3 = spine.get_action_id(&command);

        // Action IDs should be deterministic (same command produces same ID)
        prop_assert_eq!(action_id1, action_id2);
        prop_assert_eq!(action_id2, action_id3);
    }
}

// ============================================================================
// Property 4: Проверка предусловий команд
// ============================================================================

// Feature: editor-apps-proof-lane-closure, Property 4: Проверка предусловий команд
// **Validates: Requirements 2.2**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_preconditions_checked_before_execution(
        command in arbitrary_command(),
        has_project in any::<bool>(),
        has_world in any::<bool>(),
        has_selection in any::<bool>(),
    ) {
        let mut spine = CommandSpine::with_state(has_project, has_world, has_selection);

        // Check preconditions
        let precondition_result = spine.check_preconditions(&command);

        // Execute command
        let execution_result = spine.dispatch_command(command.clone());

        // If preconditions fail, execution must also fail
        if precondition_result.is_err() {
            prop_assert!(execution_result.is_err());
        }

        // If execution succeeds, preconditions must have passed
        if execution_result.is_ok() {
            prop_assert!(precondition_result.is_ok());
        }
    }
}

// Feature: editor-apps-proof-lane-closure, Property 4: Проверка предусловий команд
// **Validates: Requirements 2.2**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_project_commands_require_project(
        cmd_type in prop_oneof![
            Just(CommandType::SaveProject),
            Just(CommandType::LoadProject),
            Just(CommandType::CreateMaterial),
            Just(CommandType::AssignTexture),
        ]
    ) {
        // Without project
        let spine_no_project = CommandSpine::with_state(false, false, false);
        let command = Command::new(cmd_type.clone());
        let result = spine_no_project.check_preconditions(&command);
        prop_assert!(result.is_err());

        // With project
        let spine_with_project = CommandSpine::with_state(true, false, false);
        let command2 = Command::new(cmd_type);
        let result2 = spine_with_project.check_preconditions(&command2);
        // May still fail if other preconditions are not met, but should not fail on project
        if let Err(e) = result2 {
            prop_assert!(!e.failed_preconditions.contains(&"No project loaded".to_string()));
        }
    }
}

// Feature: editor-apps-proof-lane-closure, Property 4: Проверка предусловий команд
// **Validates: Requirements 2.2**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_transform_commands_require_selection(
        cmd_type in prop_oneof![
            Just(CommandType::TranslateEntity),
            Just(CommandType::RotateEntity),
            Just(CommandType::ScaleEntity),
        ]
    ) {
        // Without selection
        let spine_no_selection = CommandSpine::with_state(true, true, false);
        let command = Command::new(cmd_type.clone());
        let result = spine_no_selection.check_preconditions(&command);
        prop_assert!(result.is_err());

        // With selection
        let spine_with_selection = CommandSpine::with_state(true, true, true);
        let command2 = Command::new(cmd_type);
        let result2 = spine_with_selection.check_preconditions(&command2);
        prop_assert!(result2.is_ok());
    }
}

// Feature: editor-apps-proof-lane-closure, Property 4: Проверка предусловий команд
// **Validates: Requirements 2.2**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_terrain_commands_require_project_and_world(
        cmd_type in prop_oneof![
            Just(CommandType::SculptTerrain),
            Just(CommandType::PaintTerrain),
            Just(CommandType::ImportHeightmap),
        ]
    ) {
        // Without project or world
        let spine_no_state = CommandSpine::with_state(false, false, false);
        let command = Command::new(cmd_type.clone());
        let result = spine_no_state.check_preconditions(&command);
        prop_assert!(result.is_err());

        // With project and world
        let spine_with_state = CommandSpine::with_state(true, true, false);
        let command2 = Command::new(cmd_type);
        let result2 = spine_with_state.check_preconditions(&command2);
        prop_assert!(result2.is_ok());
    }
}

// ============================================================================
// Property 5: Поддержание истории команд
// ============================================================================

// Feature: editor-apps-proof-lane-closure, Property 5: Поддержание истории команд
// **Validates: Requirements 2.3**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_successful_commands_added_to_history(command in arbitrary_command()) {
        let mut spine = CommandSpine::with_state(true, true, true);

        let initial_history_len = spine.command_history().len();

        // Execute command
        let result = spine.dispatch_command(command.clone());

        // If command succeeded, it should be in history
        if result.is_ok() {
            prop_assert_eq!(spine.command_history().len(), initial_history_len + 1);
            prop_assert_eq!(spine.command_history().last().unwrap().command.id, command.id);
        }
    }
}

// Feature: editor-apps-proof-lane-closure, Property 5: Поддержание истории команд
// **Validates: Requirements 2.3**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_undo_stack_grows_with_successful_commands(commands in prop::collection::vec(arbitrary_command(), 1..10)) {
        let mut spine = CommandSpine::with_state(true, true, true);

        let mut successful_count = 0;
        for command in commands {
            if spine.dispatch_command(command).is_ok() {
                successful_count += 1;
            }
        }

        // Undo stack should have all successful commands
        prop_assert_eq!(spine.undo_count(), successful_count);
    }
}

// Feature: editor-apps-proof-lane-closure, Property 5: Поддержание истории команд
// **Validates: Requirements 2.3**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_redo_stack_cleared_on_new_command(commands in prop::collection::vec(arbitrary_command(), 2..5)) {
        let mut spine = CommandSpine::with_state(true, true, true);

        // Execute some commands
        for command in &commands[..commands.len()-1] {
            let _ = spine.dispatch_command(command.clone());
        }

        // Undo some commands
        let undo_count = std::cmp::min(2, spine.undo_count());
        for _ in 0..undo_count {
            let _ = spine.undo();
        }

        // Redo stack should have items
        let redo_count_before = spine.redo_count();
        if redo_count_before > 0 {
            // Execute a new command
            let _ = spine.dispatch_command(commands.last().unwrap().clone());

            // Redo stack should be cleared
            prop_assert_eq!(spine.redo_count(), 0);
        }
    }
}

// Feature: editor-apps-proof-lane-closure, Property 5: Поддержание истории команд
// **Validates: Requirements 2.3**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_undo_moves_to_redo_stack(commands in prop::collection::vec(arbitrary_command(), 1..5)) {
        let mut spine = CommandSpine::with_state(true, true, true);

        // Execute commands
        let mut successful_count = 0;
        for command in commands {
            if spine.dispatch_command(command).is_ok() {
                successful_count += 1;
            }
        }

        if successful_count > 0 {
            let undo_count_before = spine.undo_count();
            let redo_count_before = spine.redo_count();

            // Undo one command
            let _ = spine.undo();

            // Undo stack should decrease, redo stack should increase
            prop_assert_eq!(spine.undo_count(), undo_count_before - 1);
            prop_assert_eq!(spine.redo_count(), redo_count_before + 1);
        }
    }
}

// Feature: editor-apps-proof-lane-closure, Property 5: Поддержание истории команд
// **Validates: Requirements 2.3**
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_redo_moves_to_undo_stack(commands in prop::collection::vec(arbitrary_command(), 1..5)) {
        let mut spine = CommandSpine::with_state(true, true, true);

        // Execute commands
        for command in commands {
            let _ = spine.dispatch_command(command);
        }

        // Undo some commands
        let undo_count = std::cmp::min(2, spine.undo_count());
        for _ in 0..undo_count {
            let _ = spine.undo();
        }

        if spine.redo_count() > 0 {
            let undo_count_before = spine.undo_count();
            let redo_count_before = spine.redo_count();

            // Redo one command
            let _ = spine.redo();

            // Redo stack should decrease, undo stack should increase
            prop_assert_eq!(spine.redo_count(), redo_count_before - 1);
            prop_assert_eq!(spine.undo_count(), undo_count_before + 1);
        }
    }
}

// Feature: editor-apps-proof-lane-closure, Property 5: Поддержание истории команд
// **Validates: Requirements 2.3**
#[test]
fn prop_cannot_undo_when_stack_empty() {
    let mut spine = CommandSpine::new();

    // Undo should fail when stack is empty
    let result = spine.undo();
    assert!(result.is_err());
}

// Feature: editor-apps-proof-lane-closure, Property 5: Поддержание истории команд
// **Validates: Requirements 2.3**
#[test]
fn prop_cannot_redo_when_stack_empty() {
    let mut spine = CommandSpine::new();

    // Redo should fail when stack is empty
    let result = spine.redo();
    assert!(result.is_err());
}
