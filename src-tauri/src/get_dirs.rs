use fs_extra::dir::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn get_dirs(dirs: &mut HashMap<String, u64>, path: &str) {
    let path = Path::new(path);
    let entries = fs::read_dir(path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        println!(
            "kush foldrerrr {:?} and size {}",
            entry.path(),
            get_size(&entry.path()).unwrap_or(0000)
        );
        dirs.insert(
            entry.path().display().to_string(),
            get_size(&entry.path()).unwrap_or(0000),
        );
    }
    println!("kush done");
}
