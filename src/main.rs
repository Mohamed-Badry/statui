use color_eyre::{Result, eyre::Ok};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::backend::CheckResult;
use crate::config::load_config;

mod backend;
mod config;

const RESULT_BUFFER_SIZE: usize = 100;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let conf = load_config().unwrap();

    println!("{:#?}", conf);

    let (tx, mut rx): 
        (Sender<CheckResult>, Receiver<CheckResult>) = mpsc::channel(RESULT_BUFFER_SIZE);

    tokio::spawn(async move {
        backend::run_backend(conf, tx).await;
    });


    while let Some(result) = rx.recv().await {
        println!("[RESULT]\n{:#?}", result);
    };

    Ok(())
}
