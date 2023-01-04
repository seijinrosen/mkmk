use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn run(arg: &String) -> Result<(), Box<dyn Error>> {
    let path = Path::new(arg);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
        // eprintln!("mkdir error: {:?} {}", parent, e);
    }

    File::create(path)?;
    // eprintln!("touch error: {:?} {}", path, e);

    Ok(())
}
