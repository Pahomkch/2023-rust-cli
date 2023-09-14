use anyhow::{Context, Result};
use clap::Parser;
use log::{info, trace, warn};
extern crate log;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    env_logger::init();
    trace!("Start trace");
    let args = Cli::parse();

    let file_content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    info!("file_content readed");
    rust_grrs::find_match(&file_content, &args.pattern, &mut std::io::stdout());
    info!("Success");
    warn!("But be ready..");
    Ok(())
}
