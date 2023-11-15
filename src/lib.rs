use std::{error::Error, ffi::OsStr, fs, path::PathBuf};

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
        for (index, entry) in WalkDir::new(&config.path)
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
            .enumerate()
        {
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
    }

    Ok(())
}
