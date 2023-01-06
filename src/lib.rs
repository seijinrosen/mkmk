use std::fs;
use std::fs::File;
use std::path::Path;

use anyhow::bail;
use anyhow::Result;

pub fn run(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            bail!("mkdir: {:?} {}", parent, e);
        }
    }

    if let Err(e) = File::create(path) {
        bail!("touch: {:?} {}", path, e);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touch_ok() {
        let path = Path::new("mock_file.txt");
        assert!(!path.exists());
        let result = run(path).unwrap();
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
        let result = run(path).unwrap();
        assert_eq!(result, ());
        assert!(parent.is_dir());
        assert!(path.is_file());
        fs::remove_dir_all("mock_dir").unwrap();
        assert!(!parent.exists());
        assert!(!path.exists());
    }
}
