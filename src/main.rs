mod errors;
mod repl;

fn main() {
    loop {
        let mut buffer = repl::InputBuffer::new();
        repl::run(&mut buffer);
    }
}
