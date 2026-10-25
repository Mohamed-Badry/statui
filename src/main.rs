mod actions;
mod app;
mod backend;
mod config;
mod keymap;
mod shellexpand;
mod state;
mod ui;

use std::env;

use color_eyre::Result;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::backend::CheckResult;
use crate::config::StatuiConfig;
use crate::state::App;

const RESULT_BUFFER_SIZE: usize = 100;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    color_eyre::config::HookBuilder::default()
        .display_location_section(false)
        .install()?;

    let conf = StatuiConfig::build(env::args())?;

    let (tx, rx): (Sender<CheckResult>, Receiver<CheckResult>) = mpsc::channel(RESULT_BUFFER_SIZE);

    let conf_clone = conf.clone();
    tokio::spawn(async move {
        backend::run_backend(conf_clone, tx).await;
    });

    let mut terminal = ratatui::init();
    let mut application = App::new(&conf.endpoints);
    let app_result = app::run_app(&mut application, &mut terminal, rx).await;

    ratatui::restore();
    app_result
}
