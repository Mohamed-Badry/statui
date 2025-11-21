mod app;
mod backend;
mod config;
mod ui;
mod state;

use color_eyre::Result;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::state::App;
use crate::backend::CheckResult;
use crate::config::load_config;


const RESULT_BUFFER_SIZE: usize = 100;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let conf = load_config().unwrap();

    // println!("{:#?}", conf);
    let (tx, rx):
    (Sender<CheckResult>, Receiver<CheckResult>) = mpsc::channel(RESULT_BUFFER_SIZE);

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
