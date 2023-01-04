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
