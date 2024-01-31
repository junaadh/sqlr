use crate::errors::Errors;

use super::InputBuffer;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statements {
    Select,
    Insert,
    Create,
    Null,
    NoInput,
}

impl Statements {
    pub fn handle(buf: &str) -> Statement {
        let op = buf.split_whitespace().next().unwrap_or("no_input");
        let opcode = Self::__handle(op);
        match opcode {
            Self::Select => Statement::new(opcode, op, Vec::new()),
            Self::Insert => Statement::new(opcode, op, Vec::new()),
            Self::Create => Statement::new(opcode, op, Vec::new()),
            Self::Null => Statement::new(opcode, op, Vec::new()),
            Self::NoInput => Statement::new(opcode, op, Vec::new()),
        }
    }
    fn __handle(op: &str) -> Self {
        match op {
            "select" => Self::Select,
            "insert" => Self::Insert,
            "create" => Self::Create,
            "no_input" => Self::NoInput,
            _ => Self::Null,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
    prepare_statements(buffer);
    process_statements(buffer);
}

fn prepare_statements(buffer: &mut InputBuffer) {
    match buffer.statement.opcode {
        Statements::Null => {
            Errors::handler(
                Errors::UnrecognizedCommand,
                Some(buffer.statement.opcode_string.as_str()),
            );
        }
        Statements::NoInput => (),
        Statements::Create => (),
        Statements::Insert => (),
        _ => {
            println!("You have evoked the {}", buffer.statement.opcode_string);
        }
    }
}

fn process_statements(buffer: &mut InputBuffer) {
    match buffer.statement.opcode {
        Statements::Insert => create_value_buf(buffer),
        Statements::Create => create_value_buf(buffer),
        _ => {}
    }
}

fn create_value_buf(buffer: &mut InputBuffer) {
    let split: Vec<&str> = buffer.buffer.split(' ').collect();
    let string: Vec<String> = split.into_iter().map(|x| x.to_string()).collect();
    buffer.statement.state_buffer = string;
}
