use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};

use ratatui::widgets::TableState;

use crate::{
    backend::{CheckResult, CheckStatus},
    config::Endpoint,
};

const MAX_LATENCY_HISTORY: usize = 100;

#[derive(PartialEq, Eq)]
pub enum AppMode {
    Normal,
    Inspecting,
}

pub struct App {
    // TODO: might have to refactor into a separate AppState
    // struct if it gets too big
    pub endpoint_order: Vec<String>,
    pub endpoint_states: HashMap<String, EndpointState>,
    pub table_state: TableState,
    pub mode: AppMode,
}

impl App {
    pub fn new(endpoints: &Vec<Endpoint>) -> Self {
        let mut endpoint_order = Vec::new();
        let mut endpoint_states = HashMap::new();
        let mut table_state = TableState::default();

        table_state.select(Some(0));

        for endpoint in endpoints {
            let endpoint_state = EndpointState {
                name: endpoint.name.clone(),
                url: endpoint.url.clone(),

                // TODO: refactor method handling to have a global default
                // This will require changing backend.rs and config.rs
                method: endpoint.method.clone().unwrap_or("GET".to_string()),

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
            table_state,
            mode: AppMode::Normal,
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

    pub fn next_row(&mut self) {
        if self.endpoint_order.is_empty() {
            return;
        }

        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.endpoint_order.len() - 1 {
                    0 // Wrap around to top
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous_row(&mut self) {
        if self.endpoint_order.is_empty() {
            return;
        }

        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.endpoint_order.len() - 1 // Wrap around to bottom
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn toggle_inspect(&mut self) {
        match self.mode {
            AppMode::Normal => self.mode = AppMode::Inspecting,
            AppMode::Inspecting => self.mode = AppMode::Normal,
        };
    }
}

/// The state of an Endpoint
///
/// Keeps track of the latest status, latency and latency history.
pub struct EndpointState {
    pub name: String,
    pub url: String,
    pub method: String,

    pub latest_status: Option<CheckStatus>,
    pub latest_latency: Option<Duration>,
    pub latency_history: VecDeque<u64>,
}
