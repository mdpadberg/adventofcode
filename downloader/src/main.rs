use anyhow::Result;
mod cli;
mod http;
mod download;

fn main() -> Result<()> {
    cli::parse()
}

