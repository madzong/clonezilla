use dialoguer::{Confirm, Select};
use console::{Term, style};

use crate::menu::menu_operation;

mod utils;
mod menu;
mod types;
mod consts;

fn main() -> anyhow::Result<()> {
    let term = Term::stdout();

    let operation = menu_operation(&term)?;

    println!("{operation:?}");

    Ok(())
}
