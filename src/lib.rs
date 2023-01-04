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

// テストのサンプル

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
