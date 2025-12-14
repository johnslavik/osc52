use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, write::EncoderWriter};
use clap::{CommandFactory, Parser};
use std::{
    io::{self, IsTerminal, Read, Write},
    path::PathBuf,
};

/// Copy to clipboard using ANSI OSC52 sequence
#[derive(Parser)]
struct Args {
    /// File to read (stdin unless TTY by default)
    filename: Option<PathBuf>,
}

fn copy_osc52<R: Read, W: Write>(mut input: R, mut output: W) -> Result<()> {
    write!(output, "\x1b]52;c;")?;

    {
        let mut encoder = EncoderWriter::new(&mut output, &STANDARD);
        io::copy(&mut input, &mut encoder)?;
        encoder.finish()?;
    }

    write!(output, "\x07")?;
    output.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.filename.is_none() && io::stdin().is_terminal() {
        Args::command().print_help()?;
        return Ok(());
    }

    let stdout = io::stdout();
    let stdin = io::stdin();

    if let Some(path) = &args.filename {
        let file = std::fs::File::open(path)?;
        copy_osc52(file, stdout.lock())?;
    } else {
        copy_osc52(stdin.lock(), stdout.lock())?;
    }

    Ok(())
}
