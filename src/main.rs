use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("ヘルプメッセージ");
        process::exit(0);
    }

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if let Err(e) = run(arg) {
            println!("error: {}: {}", arg, e);
        }
    }
}

fn run(arg: &String) -> Result<(), Box<dyn Error>> {
    let path = Path::new(arg);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
        // println!("mkdir error: {:?} {}", parent, e);
    }

    File::create(path)?;
    // println!("touch error: {:?} {}", path, e);

    Ok(())
}
