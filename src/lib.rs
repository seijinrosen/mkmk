use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use anyhow::bail;
use anyhow::Result;

pub fn run(paths: Vec<PathBuf>) -> i32 {
    let mut exit_code = 0;

    for path in paths {
        if let Err(e) = path.mkmk() {
            eprintln!("Error: {}", e);
            exit_code = 1;
        }
    }

    exit_code
}

pub trait Mkmk {
    fn mkmk(&self) -> Result<()>;
}

impl Mkmk for Path {
    fn mkmk(&self) -> Result<()> {
        if let Some(parent) = self.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                bail!("mkdir: {:?} {}", parent, e);
            }
        }

        if let Err(e) = File::create(self) {
            bail!("touch: {:?} {}", self, e);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touch_ok() {
        let path = Path::new("mock_file.txt");
        assert!(!path.exists());
        let result = path.mkmk().unwrap();
        assert_eq!(result, ());
        assert!(path.is_file());
        fs::remove_file(path).unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn mkdir_ok() {
        let path = Path::new("mock_dir/mock_file.txt");
        let parent = path.parent().unwrap();
        assert!(!parent.exists());
        assert!(!path.exists());
        let result = path.mkmk().unwrap();
        assert_eq!(result, ());
        assert!(parent.is_dir());
        assert!(path.is_file());
        fs::remove_dir_all("mock_dir").unwrap();
        assert!(!parent.exists());
        assert!(!path.exists());
    }
}
