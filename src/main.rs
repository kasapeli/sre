use clap::{Parser, ValueEnum};
use colored::{self, Colorize};
use serde::Deserialize;
use std::{env, fs, io, path::Path, process::Command};

#[derive(ValueEnum, Clone)]
enum Channels {
    Stable,
    Unstable,
}

#[derive(Parser)]
struct Args {
    /// Remove a package
    #[arg(long = "remove", value_name = "PACKAGE")]
    pkgtd: Option<String>,

    /// Specifies the package to install
    #[arg(long = "install", value_name = "PACKAGE")]
    package: Option<String>,

    /// Specifies the channel to install packages from
    #[arg(long = "channel", value_name = "CHANNEL")]
    channel: Channels,
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

    if let Some(pkgtd) = args.pkgtd {
        match args.channel {
            Channels::Unstable => remove_unstable(&pkgtd),
            Channels::Stable => remove_stable(&pkgtd),
        }
    } else {
        match args.channel {
            Channels::Unstable => {
                if let Some(pkg) = args.package {
                    install_unstable(&pkg);
                }
            }
            Channels::Stable => {
                if let Some(pkg) = args.package {
                    install_stable(&pkg);
                }
            }
        }
    }
}

fn install_unstable(pkg: &String) {
    // Fetching
    let pkg_fetch_url = format!(
        "https://raw.githubusercontent.com/sre-repo/bin/refs/heads/main/unstable/{pkg}/package.toml"
    );

    println!("Trying to fetch {pkg} from {pkg_fetch_url}...");
    println!("Channel: Unstable");

    let home = env::var("HOME").expect("Somehow could not find HOME environment");
    let path = format!("{home}/.srepkgs/{pkg}");

    Command::new("wget")
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
                "---------------------------------------"
            );

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
                        "------------------------------------"
                    );
                    println!("{}", build_info);
                    println!(
                        "{}{}",
                        "[ENTER]".black().on_white(),
                        "-------------------------------------------------------------------------------"
                    );

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

fn install_stable(pkg: &str) {
    // Fetching
    let pkg_fetch_url = format!(
        "https://raw.githubusercontent.com/sre-repo/bin/refs/heads/main/stable/{pkg}/package.toml"
    );

    println!("Trying to fetch {pkg} from {pkg_fetch_url}...");
    println!("Channel: Stable");

    let home = env::var("HOME").expect("Somehow could not find HOME environment");
    let path = format!("{home}/.srepkgs/{pkg}");

    Command::new("wget")
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
                "----------------------------------------------"
            );
            println!("Name: {}", pkginfo.info.name);
            println!("Version: {}", pkginfo.info.version);
            println!("Description: {}", pkginfo.info.description);
            println!("Source: {}", pkginfo.info.source);
            println!(
                "{}{}",
                "[y/n]".black().on_white(),
                "---------------------------------------------------------------------------------------"
            );

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
                        "------------------------------------"
                    );
                    println!("{}", build_info);
                    println!(
                        "{}{}",
                        "[ENTER]".black().on_white(),
                        "-------------------------------------------------------------------------------"
                    );

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

fn remove_unstable(pkg: &String) {
    let home = env::var("HOME").expect("Somehow could not find HOME environment");
    let package_to_delete = format!("{}/.srepkgs/{}", home, pkg);

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
        _ => {}
    }
}

fn remove_stable(pkg: &String) {
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
