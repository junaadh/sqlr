use crate::errors::Errors;

use super::InputBuffer;

#[derive(Debug, PartialEq, Eq)]
pub enum Statements {
    Select,
    Insert,
    Null,
}

impl Statements {
    pub fn handle(buf: &str) -> Statement {
        let op = buf.split_whitespace().next().unwrap();
        let opcode = Self::__handle(op);
        match opcode {
            Self::Select => Statement::new(opcode, op, Vec::new()),
            Self::Insert => Statement::new(opcode, op, Vec::new()),
            Self::Null => Statement::new(opcode, op, Vec::new()),
        }
    }
    fn __handle(op: &str) -> Self {
        match op {
            "select" => Self::Select,
            "insert" => Self::Insert,
            _ => Self::Null,
        }
    }
}

pub struct Statement {
    pub opcode: Statements,
    pub opcode_string: String,
    pub state_buffer: Vec<String>,
}

impl Statement {
    pub fn new(opcode: Statements, buf: &str, state_buffer: Vec<String>) -> Self {
        Self {
            opcode,
            opcode_string: buf.to_string(),
            state_buffer,
        }
    }

    pub fn place_holder() -> Self {
        Self {
            opcode: Statements::Null,
            opcode_string: String::new(),
            state_buffer: Vec::new(),
        }
    }
}

pub fn evaluate_statements(buffer: &mut InputBuffer) {
    let buf = buffer.buffer.trim().to_string();
    buffer.buffer = buf.clone();
    let statement = Statements::handle(&buf);
    buffer.statement = statement;
}

pub fn prepare_statements(buffer: &mut InputBuffer) {
    if buffer.statement.opcode != Statements::Null {
        println!("You have evoked the {}", buffer.statement.opcode_string);
    } else {
        Errors::handler(
            Errors::UnrecognizedCommand,
            Some(buffer.statement.opcode_string.as_str()),
        );
    }
}
