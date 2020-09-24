/// Simple Git implementation with Rust.

use anyhow::Result;
use clap::Clap;
use rgit::data;

/// Basic implmentation of Git.
#[derive(Clap)]
#[clap(version = "0.0.1", author = "Macklan Weinstein <wolfgangwazzlestrauss@gmail.com>")]
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

    match opts.subcmd {
        SubCommand::Commit => println!("Hello commit!"),
        SubCommand::Init => data::init()?,
    }

    Ok(())
}
