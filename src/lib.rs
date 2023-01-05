use std::fs;
use std::fs::File;
use std::path::Path;

use anyhow::bail;
use anyhow::Result;

pub fn run(arg: &String) -> Result<()> {
    let path = Path::new(arg);

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
        let arg = String::from("mock_file.txt");
        let path = Path::new(&arg);
        assert!(!path.exists());
        let result = run(&arg).unwrap();
        assert_eq!(result, ());
        assert!(path.is_file());
        fs::remove_file(path).unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn mkdir_ok() {
        let arg = String::from("mock_dir/mock_file.txt");
        let path = Path::new(&arg);
        let parent = path.parent().unwrap();
        assert!(!parent.exists());
        assert!(!path.exists());
        let result = run(&arg).unwrap();
        assert_eq!(result, ());
        assert!(parent.is_dir());
        assert!(path.is_file());
        fs::remove_dir_all("mock_dir").unwrap();
        assert!(!parent.exists());
        assert!(!path.exists());
    }
}
