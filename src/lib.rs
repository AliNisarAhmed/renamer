use std::{error::Error, ffi::OsStr, fs, io::stdin, path::PathBuf};

use clap::Parser;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(
        short = 'p',
        long,
        help = "The path of the directory where the files are present"
    )]
    path: String,

    #[arg(long, help = "Prefix to add before the filenames", required = false)]
    prefix: Option<String>,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    let prefix = config.prefix.unwrap_or("-".to_string());
    let metadata = fs::metadata(&config.path)?;
    if !metadata.is_dir() {
        return Err("path does not point to a directory".into());
    } else {
        let dir_iter: Vec<_> = WalkDir::new(&config.path)
            .max_depth(1)
            .sort_by(|a, b| {
                a.metadata()
                    .unwrap()
                    .modified()
                    .unwrap()
                    .cmp(&b.metadata().unwrap().modified().unwrap())
            })
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.metadata().unwrap().is_file())
            .collect();

        let num_files = dir_iter.len();

        println!(
            "The directory \"{}\" has {} files, are you sure you want to continue (y/n)?",
            &config.path, num_files
        );
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();

        if user_input.trim().to_lowercase() != "y" {
            println!("exiting...");
            return Ok(());
        }

        for (index, entry) in dir_iter.into_iter().enumerate() {
            let origin_path_buf = PathBuf::from(entry.path());
            let mut result_path_buf = PathBuf::from(entry.path());
            let filename = format!(
                "{:02}{}{}",
                index + 1,
                prefix,
                origin_path_buf.file_name().unwrap().to_string_lossy()
            );
            let filename = OsStr::new(&filename);
            result_path_buf.set_file_name(filename);
            fs::rename(origin_path_buf, result_path_buf)?;
        }

        println!("Successfully renamed {} files", num_files);
    }

    Ok(())
}
