use std::env;
use std::path::PathBuf;

pub struct DirStackGuard {
    old_dir: PathBuf,
}

impl DirStackGuard {
    pub fn push_dir<P: Into<PathBuf>>(new_dir: P) -> std::io::Result<Self> {
        let old_dir = env::current_dir()?;
        env::set_current_dir(new_dir.into())?;
        Ok(Self { old_dir })
    }
}

impl Drop for DirStackGuard {
    fn drop(&mut self) {
        if let Err(err) = env::set_current_dir(&self.old_dir) {
            eprintln!("Couldn't move to directory: {err}");
        }
    }
}