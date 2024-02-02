use crate::{errors::Errors, processor::pretty_print_all};

use super::InputBuffer;

pub fn evaluate_meta(buffer: &InputBuffer) -> Result<(), Errors> {
    let buf = buffer.buffer.trim();
    match buf {
        ".exit" => {
            Errors::handler(Errors::ExitSuccess, None);
            Err(Errors::ExitSuccess)
        }
        ".help" => {
            help_prompt();
            Ok(())
        }
        ".tables" => {
            // println!("{:?}", &buffer);
            pretty_print_all(buffer)
        }
        _ => {
            Errors::handler(Errors::UnrecognizedMetaCommand, Some(buf));
            Ok(())
        }
    }
}

fn help_prompt() {
    let statement = format!(
        "{}\n{}",
        "Welcome to sqlR, written in rust",
        "Supported commands: \n\tSelect\n\tInsert\n\t.help\n\t.exit"
    );
    println!("{}", statement);
}
