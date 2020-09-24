/// Simple Git implementation with Rust.

use clap::Clap;

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
    /// Create an empty RGit repository
    Init,
}


fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Init => println!("Hello world!"),
    }
}
