use color_eyre::Result;
use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
};

use tokio::sync::mpsc::Receiver;

use crate::ui;
use crate::{
    backend::{CheckResult, CheckStatus},
    config::Endpoint,
};

const MAX_LATENCY_HISTORY: usize = 100;

pub struct App {
    // TODO: might have to refactor into a separate AppState 
    // struct if it gets too big
    pub endpoint_order: Vec<String>,
    pub endpoint_states: HashMap<String, EndpointState>,
}

impl App {
    pub fn new(endpoints: &Vec<Endpoint>) -> Self {
        let mut endpoint_order = Vec::new();
        let mut endpoint_states = HashMap::new();

        for endpoint in endpoints {
            let endpoint_state = EndpointState {
                name: endpoint.name.clone(),
                url: endpoint.url.clone(),

                latest_status: None,
                latest_latency: None,
                latency_history: VecDeque::new(),
            };

            endpoint_order.push(endpoint.name.clone());
            endpoint_states.insert(endpoint.name.clone(), endpoint_state);
        }

        Self {
            endpoint_order,
            endpoint_states,
        }
    }

    /// Called when a new CheckResult is received from the backend to update the state.
    pub fn on_result(&mut self, result: CheckResult) {
        if let Some(state) = self.endpoint_states.get_mut(&result.name) {
            state.latest_status = Some(result.status);
            state.latest_latency = Some(result.latency);

            state
                .latency_history
                .push_back(result.latency.as_millis() as u64);
            if state.latency_history.len() > MAX_LATENCY_HISTORY {
                state.latency_history.pop_front();
            }
        }
    }

    /// TUI entry point that handles drawing the ui, handling input, and displaying 
    /// results of a check.
    pub async fn run_app(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
        mut rx: Receiver<CheckResult>,
    ) -> Result<()> {
        loop {
            // 1. Draw the UI
            terminal.draw(|f| ui::render_ui(f, &self))?;

            // 2. Handle input
            // Simple input handling to quit on q
            // TODO: replace with keymap and actions model later
            // to handle all types of key inputs
            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        // 'q' was pressed, so we quit
                        return Ok(());
                    }
                }
            }

            // 3. Handle messages from the backend
            // After drawing and handling input, we check for new messages.
            // We use 'try_recv' in a loop to drain the channel of all
            // pending messages without blocking.
            while let Ok(result) = rx.try_recv() {
                self.on_result(result);
            }
        }
    }
}

pub struct EndpointState {
    pub name: String,
    pub url: String,

    pub latest_status: Option<CheckStatus>,
    pub latest_latency: Option<Duration>,
    pub latency_history: VecDeque<u64>,
}
