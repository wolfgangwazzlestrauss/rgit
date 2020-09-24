/// Simple Git implementation with Rust.
use anyhow::Result;
use clap::Clap;
use rgit::data;
use std::path::Path;

/// Basic implmentation of Git.
#[derive(Clap)]
#[clap(
    version = "0.0.1",
    author = "Macklan Weinstein <wolfgangwazzlestrauss@gmail.com>"
)]
struct Opts {
    /// Increase the verbosity of messages
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Record changes to the repository
    Commit,
    /// Create an empty RGit repository
    Init,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.verbose {
        0 => (),
        1 => println!("Verbose messaging level one enabled."),
        2 => println!("Verbose messaging level two enabled."),
        _ => println!("Verbose messaging level three enabled."),
    }

    let current_dir = Path::new(".");
    let test_file = current_dir.join("Makefile.toml");

    match opts.subcmd {
        SubCommand::Commit => data::hash_file(current_dir, &test_file)?,
        SubCommand::Init => data::init(current_dir)?,
    }

    Ok(())
}
