use std::{env, fs};

pub fn list(p: &String) {
    let home = env::var("HOME").unwrap();
    let path = format!("{}/.srepkgs/{}", home, p);
    let list = fs::read_dir(&path).unwrap();

    for item in list {
        let item = item.unwrap();
        let path = item.path();
        println!("{}", path.display());
    }
}
