use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::PathBuf,
};

use bzip2::read::BzDecoder;
use clap::Parser;
use flate2::read::GzDecoder;
use tar::Archive;
use xz2::read::XzDecoder;

enum ArchiveType {
    Gzip,
    Bzip2,
    Xz,
    Unknown,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the archive to unseal.
    #[arg(required = true, index = 1)]
    input: PathBuf,
    /// Directory to unpack the archive into.
    #[arg(short, long, default_value = ".")]
    output: PathBuf,
    /// Strip the specified number of leading components from the archive.
    #[arg(short, long, default_value_t = 0)]
    strip_components: usize,
    /// Verbose output.
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut file = File::open(&args.input)?;

    // Check the magic numbers
    let mut magic = [0u8; 6];
    file.read_exact(&mut magic)?;
    let archive_type = match magic {
        [0x1f, 0x8b, ..] => ArchiveType::Gzip,
        [0x42, 0x5a, 0x68, ..] => ArchiveType::Bzip2,
        [0xfd, b'7', b'z', b'X', b'Z', 0x00] => ArchiveType::Xz,
        _ => ArchiveType::Unknown,
    };

    file.seek(SeekFrom::Start(0))?;

    let buf = BufReader::new(file);
    match archive_type {
        ArchiveType::Gzip => {
            let decoder = GzDecoder::new(buf);
            let mut archive = Archive::new(decoder);
            unseal(&mut archive, args)?;
        }
        ArchiveType::Bzip2 => {
            let decoder = BzDecoder::new(buf);
            let mut archive = Archive::new(decoder);
            unseal(&mut archive, args)?;
        }
        ArchiveType::Xz => {
            let decoder = XzDecoder::new(buf);
            let mut archive = Archive::new(decoder);
            unseal(&mut archive, args)?;
        }
        ArchiveType::Unknown => {
            eprintln!("The file is neither a gzip nor a bzip2 archive");
            std::process::exit(1);
        }
    };

    Ok(())
}

fn unseal<R: Read>(archive: &mut Archive<R>, args: Args) -> anyhow::Result<()> {
    let entries = archive.entries()?;
    for entry in entries {
        let mut entry = entry?;
        let path = entry.path()?.to_path_buf();
        if let Some(path) = strip_components_from_path(path, args.strip_components) {
            if args.verbose {
                println!("{}", path.display());
            }
            let output = args.output.join(path);
            entry.unpack(output)?;
        }
    }
    Ok(())
}

fn strip_components_from_path(path: PathBuf, components: usize) -> Option<PathBuf> {
    let mut iter = path.components().skip(components);
    let mut path: PathBuf = iter.next()?.as_os_str().into();
    for component in iter {
        path.push(component);
    }
    Some(path)
}
