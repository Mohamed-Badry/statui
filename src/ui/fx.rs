use ratatui::{Frame, layout::Rect};
use tachyonfx::{Duration, EffectManager, Interpolation, Motion, fx};

use crate::ui::theme::Theme;

pub struct FxManager {
    global_fx: EffectManager<()>,
    inspector_fx: EffectManager<()>,
}

impl FxManager {
    pub fn new() -> Self {
        FxManager {
            global_fx: EffectManager::default(),
            inspector_fx: EffectManager::default(),
        }
    }

    pub fn render_global(&mut self, frame: &mut Frame, area: Rect, elapsed: Duration) {
        self.global_fx
            .process_effects(elapsed, frame.buffer_mut(), area);
    }

    pub fn render_inspector(&mut self, frame: &mut Frame, area: Rect, elapsed: Duration) {
        self.inspector_fx
            .process_effects(elapsed, frame.buffer_mut(), area);
    }

    pub fn trigger_startup(&mut self) {
        // A nice slide_in animation I found on tachyonfx-ftl
        // https://junkdog.github.io/tachyonfx-ftl/?example=slide_in
        let c = Theme::APP_BG;
        let timer = (1000, Interpolation::Linear);
        let fx = fx::slide_in(Motion::UpToDown, 10, 0, c, timer);
        self.global_fx.add_effect(fx);
    }
    
    pub fn trigger_open_inspector(&mut self) {
        let c = Theme::PANEL_BG;
        let timer = (1000, Interpolation::SineIn);
        let fx = fx::sweep_in(Motion::LeftToRight, 0, 0, c, timer);

        self.inspector_fx.add_effect(fx);
    }
}
