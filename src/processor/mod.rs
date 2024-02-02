use crate::{
    errors::Errors,
    repl::{statements::Statements, InputBuffer},
};

use self::execution::{execute_create, execute_insert, execute_select, Table};

pub mod execution;

#[derive(Debug)]
pub struct ExecutionBuffer {
    table_vector: Vec<Table>,
}

impl ExecutionBuffer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            table_vector: Vec::new(),
        }
    }

    pub fn add_tables(&mut self, table: Table) {
        self.table_vector.push(table);
    }
}

pub fn pretty_print_all(buf: &InputBuffer) -> Result<(), Errors> {
    let tables = buf.execution.table_vector.clone();
    if tables.is_empty() {
        println!("No tables created yet");
        Err(Errors::TableNotFound)
    } else {
        for table in tables.iter() {
            table.pretty_print();
        }
        Ok(())
    }
}

pub fn execute(buffer: &mut InputBuffer) -> Result<(), Errors> {
    let opcode = buffer.statement.opcode.clone();
    match opcode {
        Statements::Create => execute_create(buffer),
        Statements::Insert => execute_insert(buffer),
        Statements::Select => execute_select(buffer),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        processor::{execution::execute_create, pretty_print_all},
        repl::{statements::evaluate_statements, InputBuffer},
    };

    use super::execution::{Table, TableHeader};

    #[test]
    fn table_adding() {
        let mut buffer = InputBuffer::default();
        let table = Table::new("test", vec![TableHeader::new("field", "type")]);
        buffer.execution.add_tables(table);
        assert!(!buffer.execution.table_vector.is_empty());
        assert_eq!(buffer.execution.table_vector.len(), 1);
        assert!(pretty_print_all(&buffer).is_ok());
    }

    #[test]
    fn table_adding_via_create() {
        let mut buffer = InputBuffer::default();
        buffer.buffer.push_str("create table test ( field type )");
        evaluate_statements(&mut buffer);
        let table = Table::new("test", vec![TableHeader::new("field", "type")]);
        let result = execute_create(&mut buffer);
        assert!(result.is_ok());
        assert!(!buffer.execution.table_vector.is_empty());
        assert_eq!(buffer.execution.table_vector.len(), 1);
        assert_eq!(buffer.execution.table_vector.first().unwrap(), &table);
        println!("{:?}", &buffer);
        assert!(pretty_print_all(&buffer).is_ok());
    }
}
