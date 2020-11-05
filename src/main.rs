/// Simple Git implementation with Rust.
use anyhow::Result;
use clap::Clap;
use rgit::object;
use rgit::object::ObjectType;
use rgit::tree;
use std::path::{Path, PathBuf};
use std::str;

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

#[derive(Clap)]
struct WriteTree {
    /// Name of folder with contents to hash
    folder: PathBuf,
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
    sub_command: SubCommand,
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
    /// fdsl
    WriteTree(WriteTree),
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

    match opts.sub_command {
        SubCommand::CatFile(cat_file) => {
            let bytes = object::cat_file(current_dir, &cat_file.object)?;
            println!("{}", str::from_utf8(&bytes)?);
        }
        SubCommand::Commit => std::unimplemented!(),
        SubCommand::HashObject(hash_object) => {
            let hash = object::hash_object(current_dir, &hash_object.file, &ObjectType::Blob)?;
            println!("{}", hash);
        }
        SubCommand::Init => rgit::init(current_dir)?,
        SubCommand::WriteTree(write_tree) => {
            let hash = tree::write_tree(current_dir, &write_tree.folder, &ObjectType::Blob)?;
            println!("{}", hash);
        }
    };

    Ok(())
}
