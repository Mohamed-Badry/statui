use crate::{state::App, ui::fx::FxManager};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    ToggleInspect,
    CycleSort,
    ToggleSortDirection,
    Search,
    Help,
}

/// Returns true if the action is 'Quit', returns false and
/// handles the action otherwise.
///
/// 'FxManager' is being passed right now for possible future animation extensibility.
/// I tried it on inspector but it got a bit dizzying to have it run every time.
pub fn handle_action(action: &Action, app: &mut App, _fx_manager: &mut FxManager) -> bool {
    use Action::*;

    match action {
        Quit => {
            return true;
        }
        MoveUp => {
            app.previous_row();
        }
        MoveDown => {
            app.next_row();
        }
        // TODO: Implement the following functions
        ToggleInspect => {
            app.toggle_inspect();

            // Uncomment for slide in animation to be added every time
            // the inspector is opened
            // if app.mode == AppMode::Inspecting {
            //     fx_manager.trigger_inspector_slide_in();
            // }
        }
        CycleSort => {
            return false;
        }
        ToggleSortDirection => {
            return false;
        }
        Search => {
            return false;
        }
        Help => {
            return false;
        }
    }
    false
}
