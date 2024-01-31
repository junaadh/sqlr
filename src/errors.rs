use std::process;

#[derive(Debug)]
pub enum Errors {
    ExitSuccess,
    UnrecognizedMetaCommand,
    UnrecognizedCommand,
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
        }
    }
}

// impl fmt::Display for Errors {}

// impl Error for ReplErrors {}
