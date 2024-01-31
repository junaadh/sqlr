use crate::{errors::Errors, processor::execute};

use self::statements::Statement;

pub mod commands;
pub mod meta;
pub mod statements;

pub struct InputBuffer {
    pub buffer: String,
    pub statement: statements::Statement,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            statement: Statement::place_holder(),
        }
    }
}

pub fn run(buffer: &mut InputBuffer) -> Result<(), Errors> {
    commands::prompt();
    commands::read(&mut buffer.buffer).unwrap_or_default();
    evaluate(buffer);
    execute(buffer)
}

fn evaluate(buffer: &mut InputBuffer) {
    let buf = buffer.buffer.trim();
    if buf.starts_with('.') {
        meta::evaluate_meta(buf);
    } else {
        statements::evaluate_statements(buffer);
    }
}
