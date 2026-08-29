use colored::Colorize;
use std::{env, fs, io};

pub fn remove_package(pkg: &String) {
    let home = env::var("HOME").expect("Somehow could not find HOME environment");
    let package_to_delete = format!("{}/.srepkgs/{}", home, pkg);
    let package_to_delete = package_to_delete.trim();

    println!(
        "{}{}",
        "Do you want to delete this package?".black().on_white(),
        "--------------------------------"
    );
    println!("{}", package_to_delete);
    println!(
        "{}{}",
        "[y/n]".black().on_white(),
        "------------------------------------------------------"
    );

    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).unwrap();
    let confirm = confirm.trim();

    match confirm {
        "y" | "Y" | "yes" => {
            println!("Deleting {pkg}");
            fs::remove_dir_all(&package_to_delete).expect("Failed to delete file");
        }
        _ => {
            println!("Aborting");
        }
    }
}
