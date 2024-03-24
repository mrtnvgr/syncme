use clap::Parser;
use path_absolutize::*;
use std::fs::{copy, create_dir_all, remove_dir, remove_file};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
struct Args {
    source: PathBuf,
    dest: PathBuf,
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    let mut args = Args::parse();

    args.source = get_absolute(&args.source);
    args.dest = get_absolute(&args.dest);

    for source_path in get_files(&args.source, false) {
        let rel_path = source_path
            .strip_prefix(&args.source)
            .expect("Failed to strip prefix");

        let dest_path = args.dest.join(rel_path);
        let dest_dir = dest_path.parent().expect("Failed to get a directory");

        if dest_path.exists() {
            continue;
        }

        if !args.dry_run {
            create_dir_all(dest_dir).expect("Failed to create dirs");
            copy(&source_path, &dest_path).expect("Failed to copy a file");
        }

        eprintln!("{source_path:?} -> {dest_path:?}");
    }

    for dest_path in get_files(&args.dest, true) {
        let rel_path = dest_path
            .strip_prefix(&args.dest)
            .expect("Failed to strip prefix");

        let source_path = args.source.join(rel_path);

        if source_path.exists() {
            continue;
        }

        if !args.dry_run {
            remove_file(&dest_path)
                .or_else(|_| remove_dir(&dest_path))
                .expect("Failed to remove a path");
        }

        eprintln!("{dest_path:?} -> /dev/null");
    }
}

fn get_absolute(path: &Path) -> PathBuf {
    path.absolutize()
        .expect("Failed to get absolute path from source")
        .to_path_buf()
}

fn get_files(path: &Path, allow_dirs: bool) -> Vec<PathBuf> {
    WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .flatten()
        .filter(|x| !x.file_type().is_dir() || allow_dirs)
        .map(|x| x.path().to_path_buf())
        .collect()
}
