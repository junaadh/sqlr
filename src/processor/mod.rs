use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::{
    errors::Errors,
    repl::{statements::Statements, InputBuffer},
};

use self::execution::{execute_create, execute_insert, Table};

pub mod execution;

// pub static mut TABLES_VECTOR: Vec<Table> = Vec::new(); // unsafe
lazy_static! {
    static ref TABLES_VECTOR: Mutex<Vec<Table>> = Mutex::new(Vec::new());
}

pub fn add_table(table: Table) {
    let mut table_vector = TABLES_VECTOR.lock().unwrap();
    table_vector.push(table);
}

// pub fn get_table(name: &str) -> Option<usize> {
//     let mut table_vector = TABLES_VECTOR.lock().unwrap();
//     if let Some(index) = table_vector
//         .iter()
//         .position(|elem| elem.name.eq_ignore_ascii_case(name))
//     {
//         Some(index)
//     } else {
//         None
//     }
// }

pub fn pretty_print_all() {
    let tables = TABLES_VECTOR.lock().unwrap();
    if tables.is_empty() {
        println!("No tables created yet");
    } else {
        for table in tables.iter() {
            table.pretty_print();
        }
    }
}

pub fn execute(buffer: &mut InputBuffer) -> Result<(), Errors> {
    let opcode = buffer.statement.opcode.clone();
    match opcode {
        Statements::Create => execute_create(buffer),
        Statements::Insert => execute_insert(buffer),
        _ => Ok(()),
    }
}
