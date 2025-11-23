use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

use crate::actions::Action;

pub type KeyMap = HashMap<(KeyCode, KeyModifiers), Action>;

/// Returns the default KeyMap
pub fn default_keymap() -> KeyMap {
    use Action::*;
    let mut m: KeyMap = HashMap::new();

    m.insert((KeyCode::Char('q'), KeyModifiers::NONE), Quit);

    // Movement
    m.insert((KeyCode::Char('j'), KeyModifiers::NONE), MoveUp);
    m.insert((KeyCode::Char('k'), KeyModifiers::NONE), MoveDown);
    m.insert((KeyCode::Up, KeyModifiers::NONE), MoveUp);
    m.insert((KeyCode::Down, KeyModifiers::NONE), MoveDown);

    // Inspect endpoint
    m.insert((KeyCode::Char('i'), KeyModifiers::NONE), Inspect);

    // Sorting
    m.insert((KeyCode::Char('s'), KeyModifiers::NONE), CycleSort);
    m.insert(
        (KeyCode::Char('S'), KeyModifiers::NONE),
        ToggleSortDirection,
    );

    // Panels and Views
    m.insert((KeyCode::Char('?'), KeyModifiers::NONE), Help);
    m.insert((KeyCode::Char('/'), KeyModifiers::NONE), Search);

    m
}

/// Translates a KeyEvent into the corresponding action.
pub fn handle_key_event(key: KeyEvent, keymap: &KeyMap) -> Option<Action> {
    let k = (key.code, key.modifiers);
    keymap.get(&k).copied()
}
