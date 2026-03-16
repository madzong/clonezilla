use console::Term;

pub fn clear(term: &Term) -> anyhow::Result<()> {
    term.clear_screen()?;
    term.write_line("########################################")?;
    term.write_line("# Epicki skrypt do odtwarzania obrazów #")?;
    term.write_line("#           Kółko Linuxiarzy©          #")?;
    term.write_line("########################################")?;
    term.write_line("\n")?;

    Ok(())
}
