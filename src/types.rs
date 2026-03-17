use console::Term;
use dialoguer::Select;

use crate::commands::{get_disk_size, get_fs_type, get_partitions};

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

#[derive(Debug)]
pub struct Disk {
    name: String,
    size: u64,
    children: Option<Vec<Partition>>,
}

impl Disk {
    pub fn from_name(name: &str) -> anyhow::Result<Self> {
        let size = get_disk_size(name)?;
        let parts = get_partitions(name)?;

        let mut children = None;
        if let Some(parts) = parts {
            children = Some(vec![]);
            for part in parts {
                children
                    .as_mut()
                    .unwrap()
                    .push(Partition::from_name(&part)?);
            }
        }

        Ok(Self {
            name: name.to_string(),
            size,
            children,
        })
    }
}

#[derive(Debug)]
pub struct Partition {
    name: String,
    size: u64,
    fs_type: Option<String>,
}

impl Partition {
    pub fn from_name(name: &str) -> anyhow::Result<Self> {
        let size = get_disk_size(name)?;
        let fs_type = get_fs_type(name)?;

        Ok(Self {
            name: name.to_string(),
            size,
            fs_type,
        })
    }
}
