use std::io::{self, Write};

fn main() {
    // ESC c = Full terminal reset
    print!("\x1Bc");
    // Make sure it's flushed immediately
    io::stdout().flush().unwrap();
}
