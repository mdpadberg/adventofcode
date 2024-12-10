use anyhow::Result;
mod cli;
mod runner;

fn main() -> Result<()> {
  cli::parse()
}
