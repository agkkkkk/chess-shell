use std::path::PathBuf;

pub enum DisplayMode {
    Default,
    ASCII,
}

pub fn home_dir() -> Result<PathBuf, &'static str> {
    match dirs::home_dir() {
        Some(dir) => Ok(dir),
        None => Err("Could not get home directory."),
    }
}
