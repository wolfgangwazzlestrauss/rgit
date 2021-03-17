/// Simple Git implementation with Rust.
use anyhow::Result;
use clap::Clap;
use rgit::object::ObjectType;
use rgit::{commit, object, tree};
use std::path::{Path, PathBuf};
use std::str;

#[derive(Clap)]
struct CatFile {
    /// Name of object to show
    object: String,
}

#[derive(Clap)]
struct Commit {
    /// Name of folder with contents to commit
    prefix: PathBuf,
    /// Commit message
    message: String,
}

#[derive(Clap)]
struct HashObject {
    /// Name of file to hash
    file: PathBuf,
}

#[derive(Clap)]
struct ReadTree {
    /// Name of directory associated object to show
    tree_ish: String,
}

#[derive(Clap)]
struct WriteTree {
    /// Name of folder with contents to hash
    prefix: PathBuf,
}

#[derive(Clap)]
#[clap(
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
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
    Commit(Commit),
    /// Compute object ID value with contents of named file
    HashObject(HashObject),
    /// Create an empty RGit repository
    Init,
    /// Reads tree information into the index
    ReadTree(ReadTree),
    /// Create a tree object from the current index
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
            let bytes = object::cat_file(current_dir, &cat_file.object.as_bytes())?;
            println!("{}", str::from_utf8(&bytes)?);
        }
        SubCommand::Commit(commit_) => {
            let hash = commit::commit(current_dir, &commit_.prefix, &commit_.message)?;
            println!("{}", str::from_utf8(&hash)?);
        }
        SubCommand::HashObject(hash_object) => {
            let hash = object::hash_file(current_dir, &hash_object.file, &ObjectType::Blob)?;
            println!("{}", str::from_utf8(&hash)?);
        }
        SubCommand::Init => rgit::init(current_dir)?,
        SubCommand::ReadTree(read_tree) => {
            tree::read_tree(current_dir, current_dir, &read_tree.tree_ish.as_bytes())?;
        }
        SubCommand::WriteTree(write_tree) => {
            let hash = tree::write_tree(current_dir, &write_tree.prefix)?;
            println!("{}", str::from_utf8(&hash)?);
        }
    };

    Ok(())
}
