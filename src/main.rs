use clap::Parser;
use colored::{self, Colorize};
use serde::Deserialize;
use std::{
    env, fs,
    io::{self, Write},
    path::Path,
    process::Command,
};

#[derive(Parser)]
struct Args {
    /// Remove a package
    #[arg(long = "remove", value_name = "PACKAGE")]
    pkgtd: Option<String>,

    /// Specifies the package to install (format: category/package e.g. editor/vim)
    #[arg(long = "install", value_name = "PACKAGE")]
    package: Option<String>,
}

#[derive(Deserialize)]
struct PackageFile {
    info: PackageInfo,
    build: Build,
}

#[derive(Deserialize)]
struct PackageInfo {
    name: String,
    description: String,
    version: String,
    source: String,
}

#[derive(Deserialize)]
struct Build {
    inst: String,
}

fn main() {
    let args = Args::parse();

    match (args.pkgtd, args.package) {
        (Some(pkgtd), _) => {
            remove_package(&pkgtd);
        }
        (None, Some(pkg)) => {
            install_package(&pkg);
        }
        (None, None) => {
            println!("no argument provided. --help for help");
        }
    }
}

fn install_package(pkg: &String) {
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

fn read_package_info<P: AsRef<Path>>(path: P) -> Result<PackageFile, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let info: PackageFile = toml::from_str(&content)?;

    Ok(info)
}

fn remove_package(pkg: &String) {
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
