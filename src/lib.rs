use std::{env, path::PathBuf};

use config::Config;
use error::GrepError;

pub mod config;
pub mod error;
pub mod utils;

pub struct Grep {
    path: PathBuf,
    pattern: String,
    config: Config,
}

impl Grep {
    /// Creates an empty Grep object.
    pub fn new() -> Grep {
        Grep {
            path: PathBuf::new(),
            pattern: String::new(),
            config: Config::new(),
        }
    }

    /// Parses the given command line arguments and returns a Grep object.
    pub fn from(mut args: env::Args) -> Result<Grep, GrepError> {
        // Skip the executable name:
        args.next();

        let mut grep = Self::new();
        let mut arg_count: usize = 0;

        for arg in args {
            if arg.starts_with('-') {
                match arg.as_str() {
                    "-r" => grep.config.recursive = true,
                    "-rf" => grep.config.full_recursive = true,
                    "-n" => grep.config.lines = true,
                    "-c" => grep.config.count = true,
                    _ => return Err(GrepError::OptionError(arg)),
                }
                continue;
            }

            if arg_count == 0 {
                grep.pattern = arg;
                arg_count += 1;
            } else if arg_count == 1 {
                grep.path = PathBuf::from(arg);
                arg_count += 1;
            } else {
                arg_count += 1;
            }
        }

        if arg_count != 2 {
            return Err(GrepError::ArgError(2, arg_count));
        }

        Ok(grep)
    }

    /// Main entry point of grep. Error handling happens here too.
    pub fn grep(&self) {
        // -rf or -r flags are set:
        if self.config.full_recursive || self.config.recursive {
            match self.recursive() {
                Ok(_) => {}
                Err(e) => println!("{e}"),
            }
        } else {
            // -c flag is set:
            if self.config.count {
                match utils::count_matches(&self.path, &self.pattern) {
                    Ok(count) => println!("{count}"),
                    Err(e) => println!("{e}"),
                }
            } else {
                match utils::get_matches(&self.path, &self.pattern, self.config.lines, false) {
                    Ok(_) => {}
                    Err(e) => println!("{e}"),
                }
            }
        }
    }

    /// Collects all the files in a directory and calls get_matches on all of them.
    /// If the -c flag is set it counts the matches in all of the collected files.
    fn recursive(&self) -> Result<(), GrepError> {
        let file_paths = utils::collect_files_from_dir(&self.path, self.config.full_recursive)?;

        // -c flag is set:
        if self.config.count {
            println!(
                "{}",
                file_paths.iter().try_fold(0, |acc, file_path| {
                    match utils::count_matches(&file_path, &self.pattern) {
                        Ok(matches) => Ok(acc + matches),
                        Err(e) => {
                            println!("{e}");
                            Ok(acc)
                        }
                    }
                })?
            );
        } else {
            for file_path in file_paths {
                match utils::get_matches(&file_path, &self.pattern, self.config.lines, true) {
                    Ok(_) => {}
                    Err(e) => println!("{e}"),
                }
            }
        }

        Ok(())
    }
}
