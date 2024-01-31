use crate::{errors::Errors, repl::InputBuffer};

use super::TABLES_VECTOR;

pub struct TableHeader {
    pub name: String,
    pub type_of: String,
}

impl TableHeader {
    pub fn new(name: &str, type_of: &str) -> Self {
        Self {
            name: name.to_string(),
            type_of: type_of.to_string(),
        }
    }

    fn print_pretty(&self) -> String {
        format!("\t{}: {}\n", self.name, self.type_of)
    }
}

pub struct TableContent {
    pub rows: Vec<Vec<String>>,
}

impl TableContent {
    pub fn new(row: Vec<String>) -> Self {
        Self { rows: vec![row] }
    }
}

pub struct Table {
    pub name: String,
    pub headers: Vec<TableHeader>,
    pub content: Option<TableContent>,
}

impl Table {
    pub fn new(name: &str, headers: Vec<TableHeader>, content: Option<TableContent>) -> Self {
        Self {
            name: name.to_string(),
            headers,
            content,
        }
    }

    pub fn pretty_print(&self) {
        let mut headers = String::new();
        for header in self.headers.iter() {
            headers += TableHeader::print_pretty(header).as_str();
        }
        println!("Table: {}\n{headers}", self.name);
    }
}

pub fn execute_create(buffer: &InputBuffer) -> Result<(), Errors> {
    let values = buffer.statement.state_buffer.clone();
    let table = _parse_input_create_tables(values)?;
    println!("Created Table with name: {}", table.name);
    super::add_table(table);
    // for table_header in table.headers {
    //     println!("{}: {}", table_header.name, table_header.type_of);
    // }
    Ok(())
}

fn _parse_input_create_tables(buf: Vec<String>) -> Result<Table, Errors> {
    let mut headers = Vec::new();
    let mut table_name = String::new();
    let count = buf.len();
    if count < 7 {
        Errors::handler(Errors::InsufficientArguments, Some(&count.to_string()));
        return Err(Errors::InsufficientArguments);
    }

    let mut buf_iter = buf.into_iter();
    let _ = buf_iter.next();
    let keyword = buf_iter.next();
    if let Some(keyword) = keyword {
        if keyword.eq_ignore_ascii_case("table") {
            table_name = buf_iter.next().ok_or_else(|| {
                Errors::handler(Errors::InsufficientArguments, Some(" "));
                Errors::InsufficientArguments
            })?;

            let _ = buf_iter.next();

            while let Some(header_name) = buf_iter.next() {
                if header_name == ")" {
                    break;
                }
                let type_of = buf_iter.next().ok_or_else(|| {
                    Errors::handler(Errors::InsufficientArguments, Some(" "));
                    Errors::InsufficientArguments
                })?;
                let header = TableHeader::new(&header_name, &type_of);
                headers.push(header);
            }
        }
    }

    Ok(Table::new(&table_name, headers, None))
}

pub fn execute_insert(buffer: &InputBuffer) -> Result<(), Errors> {
    let values = buffer.statement.state_buffer.clone();
    _parse_input_insert_table(values.clone())?;
    println!("Successfully inserted into table: {}", values[2]);
    Ok(())
}

fn _parse_input_insert_table(buf: Vec<String>) -> Result<(), Errors> {
    let mut table_name = String::new();
    if buf.len() < 6 {
        Errors::handler(
            Errors::SyntaxError,
            Some(&format!(
                "Expected: {} colums, Recieved: {} columnss",
                "6",
                &buf.len().to_string()
            )),
        );
        return Err(Errors::SyntaxError);
    }

    let mut buf_iter = buf.clone().into_iter();
    let _ = buf_iter.next();
    let keyword = buf_iter.next();
    if let Some(keyword) = keyword {
        if keyword.eq_ignore_ascii_case("into") {
            table_name = buf_iter.next().ok_or_else(|| {
                Errors::handler(Errors::InsufficientArguments, Some(" "));
                Errors::InsufficientArguments
            })?;
        }
    }

    // let table_size = super::get_table(&table_name).ok_or_else(|| {
    //     Errors::handler(
    //         Errors::TableNotFound,
    //         Some(&format!(
    //             "table with name {}, has not been found",
    //             table_name
    //         )),
    //     );
    //     Errors::TableNotFound
    // })?;
    let mut table_vector = TABLES_VECTOR.lock().unwrap();
    let table = table_vector
        .iter_mut()
        .find(|elem| elem.name.eq_ignore_ascii_case(&table_name))
        .ok_or_else(|| {
            Errors::handler(
                Errors::TableNotFound,
                Some(&format!(
                    "table with name {}, has not been found",
                    table_name
                )),
            );
            Errors::TableNotFound
        })?;
    let no_columns = table.headers.len();
    if buf.len() - 6 != no_columns {
        Errors::handler(
            Errors::InvalidTableColums,
            Some(&format!(
                "Expected: {} colums, Recieved: {} columnss",
                &no_columns.to_string(),
                &buf.len().to_string()
            )),
        );
        return Err(Errors::InvalidTableColums);
    }

    // let mut values: Vec<String> = buf_iter.take(no_columns - 5).collect();
    let mut values = buf.clone();
    let _ = values.split_off(buf.len() - 5);
    for i in values.clone().iter() {
        println!("{i}");
    }
    if let Some(last) = values.pop() {
        if !last.eq_ignore_ascii_case(")") {
            Errors::handler(
                Errors::SyntaxError,
                Some(&format!("Expected ')' found {}", last)),
            );
            return Err(Errors::SyntaxError);
        }
    }
    let table_content = TableContent::new(values);
    table.content = Some(table_content);
    Ok(())
}
