use console::Term;
use dialoguer::Select;

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    RestoreDisk,
    RestoreParts,
}

impl Operation {
    pub fn select(term: &Term) -> anyhow::Result<Self> {
        let choice = Select::new()
            .items(&["restoredisk", "restoreparts"])
            .default(0)
            .with_prompt("Wybierz typ operacji")
            .interact_on(term)?;
        
        match choice {
            0 => Ok(Self::RestoreDisk),
            1 => Ok(Self::RestoreParts),
            _ => unreachable!(),
        }
    }
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Self::RestoreDisk => "restoredisk".to_string(),
            Self::RestoreParts => "restoreparts".to_string(),
        }
    }
}
