use console::Term;

use crate::{types::Operation, utils::clear};

pub fn menu_operation(term: &Term) -> anyhow::Result<Operation> {
    clear(term)?;
    Operation::select(term)
}
