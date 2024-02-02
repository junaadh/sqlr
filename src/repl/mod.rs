use crate::{
    errors::Errors,
    processor::{self, execute},
};

use self::statements::Statement;

pub mod commands;
pub mod meta;
pub mod statements;

#[derive(Debug)]
pub struct InputBuffer {
    pub buffer: String,
    pub statement: statements::Statement,
    pub execution: processor::ExecutionBuffer,
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            buffer: String::new(),
            statement: Statement::place_holder(),
            #[allow(clippy::box_default)]
            execution: processor::ExecutionBuffer::new(),
        }
    }
}

impl InputBuffer {
    pub fn clear(&mut self) {
        self.buffer = String::new();
        self.statement = Statement::place_holder();
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
        let _ = meta::evaluate_meta(buffer);
    } else {
        statements::evaluate_statements(buffer);
    }
}
