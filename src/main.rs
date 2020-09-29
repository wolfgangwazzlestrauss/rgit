/// Simple Git implementation with Rust.
use anyhow::Result;
use clap::Clap;
use rgit::data;
use std::path::{Path, PathBuf};

#[derive(Clap)]
struct CatFile {
    /// Name of object to show
    object: String,
}

#[derive(Clap)]
struct HashObject {
    /// Name of file to hash
    file: PathBuf,
}

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
    /// Provide content for repository objects
    CatFile(CatFile),
    /// Record changes to the repository
    Commit,
    /// Compute object ID value with contents of named file
    HashObject(HashObject),
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

    match opts.subcmd {
        SubCommand::CatFile(cat_file) => {
            let text = data::cat_file(current_dir, &cat_file.object)?;
            println!("{}", text);
        }
        SubCommand::Commit => std::unimplemented!(),
        SubCommand::HashObject(hash_object) => {
            let hash = data::hash_object(current_dir, &hash_object.file)?;
            println!("{}", hash);
        }
        SubCommand::Init => data::init(current_dir)?,
    }

    Ok(())
}
