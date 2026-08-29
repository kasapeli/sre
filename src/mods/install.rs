use colored::Colorize;
use std::{
    env,
    io::{self, Write},
    process::Command,
};

use super::package::read_package_info;

pub fn install_package(pkg: &String) {
    // Fetching
    let pkg_fetch_url = format!(
        "https://raw.githubusercontent.com/sre-repo/bin/refs/heads/main/{pkg}/package.toml"
    );

    println!("Trying to fetch {pkg} from {pkg_fetch_url}...");

    let home = env::var("HOME").expect("Somehow could not find HOME environment");
    let path = format!("{home}/.srepkgs/{pkg}");

    Command::new("wget") // TODO: replace wget here
        .arg("-q")
        .arg("-nc")
        .arg("-P")
        .arg(&path)
        .arg(&pkg_fetch_url)
        .status()
        .expect("Failed to download package.toml");

    let read_path = format!("{path}/package.toml");

    match read_package_info(read_path) {
        Ok(pkginfo) => {
            println!(
                "{}{}",
                "Do you want to download the following package?"
                    .black()
                    .on_white(),
                "---------------------------------------------"
            );
            println!("Name: {}", pkginfo.info.name);
            println!("Version: {}", pkginfo.info.version);
            println!("Description: {}", pkginfo.info.description);
            println!("Source: {}", pkginfo.info.source);
            println!(
                "{}{}",
                "[y/n]".black().on_white(),
                "--------------------------------------------------------------------------------------"
            );

            print!("> ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            let build_info = pkginfo.build.inst;

            match choice {
                "y" | "Y" | "yes" => {
                    println!(
                        "{}{}",
                        "The following command(s) or script(s) will be run:"
                            .black()
                            .on_white(),
                        "-----------------------------------------"
                    );
                    println!("{}", build_info);
                    println!(
                        "{}{}",
                        "[ANY KEY to continue | CTRL + C to abort]"
                            .black()
                            .on_white(),
                        "--------------------------------------------------"
                    );

                    print!("> ");
                    io::stdout().flush().unwrap();

                    let mut wait = String::new();
                    io::stdin().read_line(&mut wait).unwrap();

                    let install = Command::new("sh")
                        .arg("-c")
                        .arg(&build_info)
                        .status()
                        .expect("Failed to install");

                    println!("Finish with code {install}");
                }
                _ => {}
            }
        }
        Err(e) => {
            eprintln!("Error reading package.toml: {e}");
        }
    }
}
