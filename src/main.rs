use std::env::{self};
use std::fs::{self, DirEntry};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "replace" {
        replace_file_extension(&args[2]);
    } else if args[1] == "only_keep" {
        only_keep_selected_file_type(&args[2]);
    }
}

fn replace_file_extension(file_extension: &str) {
    let mut file_names: Vec<DirEntry> = Vec::new();
    let path = std::env::current_dir().unwrap();
    for entry in fs::read_dir(path).unwrap() {
        if entry.as_ref().unwrap().path().is_file() {
            file_names.push(entry.unwrap());
        } else {
            continue;
        }
    }
    for i in file_names {
        println!("{}", i.path().file_stem().unwrap().to_str().unwrap());
        fs::rename(
            i.path().file_name().unwrap().to_str().unwrap(),
            format!(
                "{}.{}",
                i.path().file_stem().unwrap().to_str().unwrap(),
                file_extension.trim().to_string()
            ),
        )
        .ok();
    }
}

fn only_keep_selected_file_type(file_extension: &str) {
    let path = std::env::current_dir().unwrap();
    for entry in fs::read_dir(path).unwrap() {
        if entry.as_ref().unwrap().path().is_file() {
            if entry
                .as_ref()
                .unwrap()
                .path()
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                != file_extension
            {
                fs::remove_file(entry.unwrap().path()).unwrap();
            } else {
                continue;
            }
        } else {
            continue;
        }
    }
}
