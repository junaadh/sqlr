use crate::{errors::Errors, processor::pretty_print_all};

pub fn evaluate_meta(buffer: &str) {
    match buffer {
        ".exit" => Errors::handler(Errors::ExitSuccess, None),
        ".help" => help_prompt(),
        ".tables" => pretty_print_all(),
        _ => Errors::handler(Errors::UnrecognizedMetaCommand, Some(buffer)),
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
