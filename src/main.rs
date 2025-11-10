use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut line = String::new();

    loop {
        print!("$ ");
        stdout.flush()?; // show prompt

        line.clear();
        let bytes = stdin_lock.read_line(&mut line)?;
        if bytes == 0 {
            // EOF (Ctrl-D): exit 0
            std::process::exit(0);
        }

        let mut parts = line.split_whitespace();
        let Some(cmd) = parts.next() else {
            continue;
        };

        match cmd {
            "exit" => {
                let code = parts
                    .next()
                    .and_then(|s| s.parse::<i32>().ok())
                    .unwrap_or(0);
                std::process::exit(code);
            }
            _ => {
                eprintln!("{cmd}: command not found");
            }
        }
    }
}
