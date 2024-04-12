use copy_dir;
use std::{env::args, fs, process::exit};

fn parse_args() -> String {
    let args = args();
    // println!("Args: {args:?}");
    if args.len() != 2 {
        println!("Not enough arguments given");
        exit(1);
    }
    args.last().expect("Can't get here without args")
}

fn bak(file_path: &str, target: &str, is_dir: bool) -> i32 {
    if is_dir {
        let res = copy_dir::copy_dir(file_path, target);
        match res {
            Ok(_) => {
                println!("Successfully backed up {} as {}", file_path, target);
                0
            }
            Err(e) => {
                println!("Error: {}", e);
                1
            }
        }
    } else {
        let res = fs::copy(file_path, target);
        match res {
            Ok(_) => {
                println!("Successfully backed up {} as {}", file_path, target);
                0
            },
            Err(e) => {
                println!("Error: {}", e);
                1
            }
        }
    }
}

fn _main<'foo>() -> i32 {
    let file_path = parse_args();
    let metadata = fs::metadata(&file_path);
    let target = format!("{}{}", &file_path, ".bak");
    match metadata {
        Ok(md) => bak(&file_path, &target, md.is_dir()),
        Err(e) => {
            println!("Error: {}", e);
            1
        }
    }
}

fn main() {
    exit(_main());
}
