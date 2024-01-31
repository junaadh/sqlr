use repl::commands::wlc_prompt;

pub mod errors;
pub mod processor;
pub mod repl;

fn main() {
    wlc_prompt("0.0.1");
    loop {
        let mut buffer = repl::InputBuffer::new();
        if repl::run(&mut buffer).is_err() {
            continue;
        };
    }
}
