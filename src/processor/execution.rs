use crate::{errors::Errors, repl::InputBuffer};

#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn print_table(&self) -> String {
        format!("{} ", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TableContent {
    pub row: Vec<String>,
}

impl TableContent {
    pub fn new(row: Vec<String>) -> Self {
        Self { row }
    }

    pub fn print_table(&self) -> String {
        let mut concat_content = String::new();
        for column in self.row.clone().into_iter() {
            concat_content += &format!("{} ", column);
        }
        concat_content += "\n";
        concat_content
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub name: String,
    pub headers: Vec<TableHeader>,
    pub content: Vec<TableContent>,
}

impl Table {
    pub fn new(name: &str, headers: Vec<TableHeader>) -> Self {
        Self {
            name: name.to_string(),
            headers,
            content: Vec::new(),
        }
    }

    pub fn blank() -> Self {
        Self {
            name: String::new(),
            headers: Vec::new(),
            content: Vec::new(),
        }
    }

    pub fn pretty_print(&self) {
        let mut headers = String::new();
        for header in self.headers.iter() {
            headers += TableHeader::print_pretty(header).as_str();
        }
        println!("Table: {}\n{headers}", self.name);
    }

    pub fn print_table(&self) {
        println!("Table: {}", self.name);
        let mut header_str = String::new();
        let mut content_str = String::new();
        for header in self.headers.clone().iter() {
            header_str += &header.print_table();
        }
        for row in self.content.clone().iter() {
            content_str += &row.print_table();
        }
        println!("{}\n{}", header_str, content_str);
    }

    pub fn update_content(&mut self, content: TableContent) {
        self.content.push(content);
    }
}

pub fn execute_create(buffer: &mut InputBuffer) -> Result<(), Errors> {
    let table = _parse_input_create_tables(buffer)?;
    buffer.execution.add_tables(table.clone());
    println!("Created Table with name: {}", table.name);
    Ok(())
}

fn _parse_input_create_tables(buffer: &mut InputBuffer) -> Result<Table, Errors> {
    let buf = buffer.statement.state_buffer.clone();
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

    let table = Table::new(&table_name, headers);
    // println!("{:?}", table);
    Ok(table)
    // Ok(Table::new(&table_name, headers, None))
}

pub fn execute_insert(buffer: &mut InputBuffer) -> Result<(), Errors> {
    _parse_input_insert_table(buffer)?;
    println!(
        "Successfully inserted into table: {}",
        buffer.statement.state_buffer[2]
    );
    Ok(())
}

fn _parse_input_insert_table(buffer: &mut InputBuffer) -> Result<(), Errors> {
    let buf = buffer.statement.state_buffer.clone();
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

    let table = buffer
        .execution
        .table_vector
        .iter_mut()
        .find(|x| x.name.eq_ignore_ascii_case(&table_name))
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
    let _ = buf_iter.next();
    let _ = buf_iter.next();
    // let mut values = buf.clone();
    // let mut new_values = values.split_off(buf.len() - 5);
    let mut new_values: Vec<String> = buf_iter.collect();
    // for i in values.clone().iter() {
    // println!("{i}");
    // }
    if let Some(last) = new_values.pop() {
        if !last.eq_ignore_ascii_case(")") {
            Errors::handler(
                Errors::SyntaxError,
                Some(&format!("Expected ')' found {}", last)),
            );
            return Err(Errors::SyntaxError);
        }
    }
    let table_content = TableContent::new(new_values);
    table.update_content(table_content);
    Ok(())
}

pub fn execute_select(buffer: &mut InputBuffer) -> Result<(), Errors> {
    _parse_select_all_table(buffer)
}

fn _parse_select_all_table(buffer: &mut InputBuffer) -> Result<(), Errors> {
    let buf = buffer.statement.state_buffer.clone();
    if buf.len() < 4 {
        Errors::handler(
            Errors::InsufficientArguments,
            Some(&format!("Expected 4< arguments recieved {}", &buf.len())),
        );
        return Err(Errors::InsufficientArguments);
    }

    let mut buf_iter = buf.clone().into_iter();
    let _ = buf_iter.next();
    if let Some(all) = buf_iter.next() {
        if !all.eq_ignore_ascii_case("*") {
            Errors::handler(Errors::SyntaxError, Some("supported only all printing rn"));
            return Err(Errors::SyntaxError);
        }
    }
    let _ = buf_iter.next();
    let table_name = buf_iter.next().ok_or_else(|| {
        Errors::handler(Errors::InsufficientArguments, Some(" "));
        Errors::InsufficientArguments
    })?;

    let table = buffer
        .execution
        .table_vector
        .iter()
        .find(|x| x.name.eq_ignore_ascii_case(&table_name))
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

    table.print_table();

    Ok(())
}
