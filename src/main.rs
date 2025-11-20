use color_eyre::{Result, eyre::Ok};
use crate::config::load_config;

mod config;

fn main() -> Result<()> {
    let conf = load_config().unwrap();

    println!("{:#?}", conf);
    Ok(())
}