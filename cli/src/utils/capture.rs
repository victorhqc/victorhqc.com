use std::io;
use std::io::Write;

pub fn capture(msg: &str) -> String {
    let mut capture = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut capture)
        .expect("Failed to capture String");

    capture.trim().to_string()
}
