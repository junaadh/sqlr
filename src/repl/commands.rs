use std::io::{self, Write};

pub fn wlc_prompt(version: &str) {
    println!("Welcome to sqlR.");
    println!(">>> Verson {version}");
}

pub fn prompt() {
    print!("\x1b[35mdb \x1b[36m> \x1b[m");
    let _ = io::stdout().flush();
}

pub fn read(buffer: &mut String) -> Result<(), ()> {
    io::stdin()
        .read_line(buffer)
        .map_err(|err| println!("{}", err))?;
    Ok(())
}
