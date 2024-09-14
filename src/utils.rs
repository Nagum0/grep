use colorama::Colored;
use std::{fs, io::ErrorKind, path::PathBuf};

use crate::error::GrepError;

/// Prints the matching lines to stdout.
/// ### Parameters
/// * file_path: Path to the given file;
/// * pattern: Pattern to match;
/// * lines: If true will include the line number next to the matched line;
/// * recursive: If true will include the file path next to the matched line;
pub fn get_matches(
    file_path: &PathBuf,
    pattern: &String,
    lines: bool,
    recursive: bool,
) -> Result<(), GrepError> {
    let file_contents = read_file(&file_path)?;
    let mut line_count: usize = 1;

    for line in file_contents.lines() {
        let mut formatted_line = String::new();

        if line.contains(pattern) {
            formatted_line.push_str(line);
        } else {
            line_count += 1;
            continue;
        }

        if lines {
            formatted_line.insert_str(0, &format!("{}:", line_count.to_string().color("red")));
        }

        if recursive {
            formatted_line.insert_str(0, &format!("{}:", pathbuf_to_string(&file_path, "blue")?));
        }
        line_count += 1;

        println!("{}", formatted_line);
    }

    Ok(())
}

/// Returns the number of matched lines.
/// ### Parameters
/// * file_path: Path to the file;
/// * pattern: Pattern to match against;
pub fn count_matches(file_path: &PathBuf, pattern: &String) -> Result<usize, GrepError> {
    Ok(read_file(&file_path)?.lines().fold(0, |mut acc, line| {
        if line.contains(pattern) {
            acc += 1;
        }
        acc
    }))
}

/// Collects all the files from a directory. If full is true will collect all
/// the files from subdirectories too.
/// ### Parameters
/// * path: Path to the directory;
/// * full: If true will collect files from subdirectories too;
pub fn collect_files_from_dir(path: &PathBuf, full: bool) -> Result<Vec<PathBuf>, GrepError> {
    let error_path_as_string = pathbuf_to_string(&path, "red")?;

    // Checking whether the given path is a directory:
    if !path.is_dir() {
        return Err(GrepError::FileError(
            error_path_as_string,
            "Not a directory",
        ));
    }

    // Getting the contents of the given directory:
    let dir = fs::read_dir(path).map_err(|e| match e.kind() {
        ErrorKind::NotFound => GrepError::FileError(error_path_as_string, "Directory not found"),
        _ => GrepError::FileError(
            error_path_as_string,
            "Error while getting directory contents",
        ),
    })?;

    let mut file_paths: Vec<PathBuf> = Vec::new();

    for entry in dir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => return Err(GrepError::EntryError),
        };
        let entry_path = entry.path();

        if entry_path.is_dir() && full {
            file_paths.extend(collect_files_from_dir(&entry_path, full)?);
        } else if entry_path.is_dir() {
            println!("{:?} : IS DIRECTORY", entry_path);
        } else {
            file_paths.push(entry_path);
        }
    }

    Ok(file_paths)
}

/// Turns a PathBuf to a String and  colors it to the given color:
fn pathbuf_to_string(file_path: &PathBuf, color: &'static str) -> Result<String, GrepError> {
    match file_path.clone().to_str() {
        Some(file_path) => Ok(file_path.to_string().color(color).to_string()),
        None => Err(GrepError::PathError(file_path.clone())),
    }
}

/// Reads the contents of a file and checks errors.
fn read_file(file_path: &PathBuf) -> Result<String, GrepError> {
    // Converting file path to String:
    let error_file_path_as_string = pathbuf_to_string(&file_path, "red")?;

    // Reading file contents:
    Ok(fs::read_to_string(&file_path).map_err(|e| match e.kind() {
        ErrorKind::NotFound => GrepError::FileError(error_file_path_as_string, "File not found"),
        ErrorKind::InvalidData => GrepError::FileError(error_file_path_as_string, "Invalid data"),
        ErrorKind::InvalidInput => {
            GrepError::FileError(error_file_path_as_string, "Unreadable input")
        }
        _ => GrepError::FileError(error_file_path_as_string, "Unknown file error"),
    })?)
}

pub fn help() {
    println!("Usage:\n\n  grep [OPTIONS...] <pattern> <path>");
    println!("  grep <pattern> [OPTIONS...] <path>");
    println!("  grep <pattern> <path> [OPTIONS...]");
    println!("");
    println!("  -r\n\tRecusively search the given directory for the given pattern.");
    println!(
        "  -rf\n\tRecusively search the given directory and subdirectories for the given pattern."
    );
    println!("  -n\n\tAppend line numbers to the output.");
    println!("  -c\n\tInstead of printing the found matches print the number of found matches.");
    println!("  -e\n\tShow errors thrown.");
    println!("  --help\n\tPrint usage information.");
}
