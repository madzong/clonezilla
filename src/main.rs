use dialoguer::{Confirm, Select};
use console::{Term, style};

fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();

    let choice = Select::new()
        .with_prompt("Wybierz partycję")
        .items(&["sda1", "sda2", "sda3"])
        .default(0)
        .interact()
        .unwrap();

    println!("{choice}");
}
