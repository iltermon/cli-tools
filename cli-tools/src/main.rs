use std::env;
use std::fs::{self, DirEntry};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "rfe" {
        replace_file_extension();
    } else if args[1] == "oksft" {
        only_keep_selected_file_type();
    }
}

fn replace_file_extension() {
    let mut file_names: Vec<DirEntry> = Vec::new();
    let path = std::env::current_dir().unwrap();
    //println!("{}", path.display());
    // println!("{}", Path::new(&path).exists());
    let mut counter = 0;
    for entry in fs::read_dir(path).unwrap() {
        // println!("{}", entry.unwrap().path().display());
        if entry.as_ref().unwrap().path().is_file() {
            // println!("{}", entry.as_ref().unwrap().path().display());
            file_names.push(entry.unwrap());
        } else {
            continue;
        }
    }
    for i in file_names {
        println!("{}", i.path().file_name().unwrap().to_str().unwrap());
    }
}

// onlyfiles = [f for f in listdir() if isfile(join(f))]
// for i in onlyfiles:
//     if os.path.splitext(i)[1] != '.py':
//         base = os.path.splitext(i)[0]
//         os.rename(i, base + '.sql')
//TODO: check if path is valid

fn only_keep_selected_file_type() {
    println!("only_keep_selected_file_type");
}
