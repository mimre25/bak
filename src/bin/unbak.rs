use std::{
    env::{args, temp_dir},
    fs,
    process::exit,
};

fn parse_args() -> String {
    let args = args();
    // println!("Args: {args:?}");
    if args.len() != 2 {
        println!("Not enough arguments given");
        exit(1);
    }
    args.last().expect("Can't get here without args")
}

struct DirHolder {
    backup_dir: String,
    dir: String,
    success: bool,
}
impl DirHolder {
    fn new(dir: &str) -> Self {
        let mut dest = temp_dir();
        dest.push(dir);
        let res = fs::rename(dir, &dest);
        match res {
            Ok(_) => DirHolder {
                backup_dir: dest.to_str().expect("aaaah").to_string(),
                dir: dir.to_string(),
                success: true,
            },
            Err(e) => {
                println!("Error cleaning up original directory: {}", e);
                exit(1);
            }
        }
    }
}

impl Drop for DirHolder {
    fn drop(&mut self) {
        if self.success {
            // println!("Removing backup dir {}", &self.backup_dir);
            let res = fs::remove_dir_all(&self.backup_dir);
            match res {
                Ok(_) => (),
                Err(e) => {
                    println!("Error cleaning up backup directory: {}", e);
                    println!("Please delete the directory {}", self.backup_dir);
                    exit(1);
                }
            }
        } else {
            // move file back
            let res = fs::rename(&self.backup_dir, &self.dir);
            match res {
                Ok(_) => (),
                Err(e) => {
                    println!("Error cleaning up backup directory: {}", e);
                    println!("You can find the directory in {}", self.backup_dir);
                    exit(1);
                }
            }
        }
    }
}
fn unbak(file_path: &str, target: &str) -> i32 {
    let res = fs::rename(&file_path, target);
    match res {
        Ok(_) => {
            println!("Successfully restored {} as {}", file_path, target);
            0
        },
        Err(e) => {
            println!("Error: {}", e);
            1
        }
    }
}

fn _main<'foo>() -> i32 {
    let file_path = parse_args();
    let metadata = fs::metadata(&file_path);
    match metadata {
        Ok(md) => {
            let target = (&file_path).strip_suffix(".bak");
            match target {
                Some(t) => {
                    // println!("Is dir: {}", md.is_dir());
                    if md.is_dir() {
                        let _dir_holder = DirHolder::new(&t);
                        unbak(&file_path, t)
                    } else {
                        unbak(&file_path, t)
                    }
                }
                None => {
                    println!("File {file_path} is not a .bak file");
                    1
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            1
        }
    }
}

fn main() {
    exit(_main());
}
