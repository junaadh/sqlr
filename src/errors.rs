use std::process;

#[derive(Debug)]
pub enum Errors {
    ExitSuccess,
    UnrecognizedMetaCommand,
    UnrecognizedCommand,
    InsufficientArguments,
    InvalidTableColums,
    TableNotFound,
    SyntaxError,
    // ParseError,
}

impl Errors {
    pub fn handler(self, buf: Option<&str>) {
        match self {
            Self::ExitSuccess => {
                println!("GoodBye!");
                process::exit(0);
            }
            Self::UnrecognizedMetaCommand => {
                println!("Unrecognized meta command: {}", buf.unwrap());
            }
            Self::UnrecognizedCommand => {
                println!("Unrecognized command: {}", buf.unwrap());
            }
            Self::InsufficientArguments => {
                println!("Insufficient amount of args: {}", buf.unwrap());
            }
            Self::InvalidTableColums => {
                println!("Invalid number of table colums: {}", buf.unwrap());
            }
            Self::TableNotFound => {
                println!("Table not found: {}", buf.unwrap());
            }
            Self::SyntaxError => {
                println!("Syntax error at: {}", buf.unwrap());
            } // Self::ParseError => {
              //     println!("Parse error");
              // }
        }
    }
}

// impl fmt::Display for Errors {}

// impl Error for ReplErrors {}
