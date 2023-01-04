use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let path = Path::new(arg);
        let parent = path.parent().unwrap();
        fs::create_dir_all(parent).unwrap();
        File::create(path).unwrap();
    }
}
