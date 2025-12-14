use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use clap::{CommandFactory, Parser};
use std::io::{self, IsTerminal, Read, Write};

macro_rules! OSC52 {
    () => {
        "\x1b]52;c;{}\x07"
    };
}

/// Copy to clipboard using ANSI OSC52 sequence
#[derive(Parser)]
struct Args {
    /// File to read (stdin unless TTY by default)
    filename: Option<String>,

    /// Don't echo the copied content
    #[arg(short, long)]
    silent: bool,

    /// Remove the trailing newline (LF) if present
    #[arg(short = 'n', long)]
    strip_newline: bool,
}

fn read(args: &Args) -> Result<Vec<u8>> {
    let mut buf = match &args.filename {
        Some(path) => std::fs::read(path)?,
        None => {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            buf
        }
    };

    if args.strip_newline && buf.last() == Some(&b'\n') {
        buf.pop();
    }

    Ok(buf)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.filename.is_none() && io::stdin().is_terminal() {
        Args::command().print_help()?;
        return Ok(());
    }

    let text = read(&args)?;
    let blob = STANDARD.encode(&text);

    let mut stdout = io::stdout();
    write!(stdout, OSC52!(), blob)?;
    stdout.flush()?;

    if !args.silent {
        println!("{:?}", std::str::from_utf8(&text)?);
    }

    Ok(())
}
