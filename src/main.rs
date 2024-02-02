use repl::commands::wlc_prompt;

pub mod errors;
pub mod processor;
pub mod repl;

fn main() {
    wlc_prompt("0.1.0");
    let mut buffer = repl::InputBuffer::default();
    loop {
        if repl::run(&mut buffer).is_err() {
            continue;
        };
    }
}
