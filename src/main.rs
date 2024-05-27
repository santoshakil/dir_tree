use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn visit_dirs(
    dir: &Path,
    exclude: Option<&PathBuf>,
    cb: &mut dyn FnMut(&fs::DirEntry),
) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();

            if name.starts_with(".")
                || name.starts_with("target")
                || name.starts_with("build")
                || name.starts_with("android")
                || name.starts_with("ios")
                || name.starts_with("macos")
                || name.starts_with("linux")
                || name.starts_with("windows")
                || name.ends_with(".lock")
            {
                continue;
            } else if let Some(exclude_path) = exclude {
                if path.starts_with(exclude_path) {
                    continue;
                }
            }

            if path.is_dir() {
                visit_dirs(&path, exclude, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!(
            "Usage: {} <path_to_project_directory> [path_to_exclude]",
            args[0]
        );
        std::process::exit(1);
    }

    let project_dir = Path::new(&args[1]);
    let exclude_dir = if args.len() == 3 {
        Some(PathBuf::from(&args[2]))
    } else {
        None
    };
    let mut output = File::create("dir_tree.txt")?;

    visit_dirs(project_dir, exclude_dir.as_ref(), &mut |entry| {
        if let Some(path_str) = entry.path().to_str() {
            writeln!(output, "{}", path_str).unwrap();
        }
    })?;

    println!("Project structure has been saved to dir_tree.txt");
    Ok(())
}
