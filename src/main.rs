use console::Term;

use crate::types::Disk;

mod utils;
mod menu;
mod types;
mod consts;
mod commands;

fn main() -> anyhow::Result<()> {
    let term = Term::stdout();

    println!("{:?}", Disk::from_name("nvme0n1"));

    Ok(())
}
