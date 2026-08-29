use clap::{Parser, ValueEnum};
use serde::Deserialize;
use std::{env, fs, io, path::Path, process::Command};

#[derive(ValueEnum, Clone)]
enum Channels {
    Stable,
    Unstable,
}

#[derive(Parser)]
struct Args {
    /// Specifies the package to install
    #[arg(long = "install", value_name = "PACKAGE")]
    package: String,

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

    match args.channel {
        Channels::Unstable => install_unstable(&args.package),
        Channels::Stable => install_stable(&args.package),
    }
}

fn install_unstable(pkg: &str) {
    // Fetching
    let pkg_fetch_url = format!(
        "https://raw.githubusercontent.com/sre-repo/bin/refs/heads/main/unstable/{pkg}/package.toml"
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
            println!("Do you want to download the following package?");
            println!("----------------------------------------------");
            println!("Name: {}", pkginfo.info.name);
            println!("Version: {}", pkginfo.info.version);
            println!("Description: {}", pkginfo.info.description);
            println!("Source: {}", pkginfo.info.source);
            println!("[y/n]-----------------------------------------");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            let build_info = pkginfo.build.inst;

            match choice {
                "y" | "Y" | "yes" => {
                    println!("The following command(s) or script(s) will be run:");
                    println!("---------------------------------------------------");
                    println!("{}", build_info);
                    println!("[ENTER]--------------------------------------------");

                    let mut wait = String::new();
                    io::stdin().read_line(&mut wait).unwrap();

                    let install = Command::new("sh")
                        .arg("-c")
                        .arg(&build_info)
                        .status()
                        .expect("Failed to install");

                    println!("Finish with code {install}");
                }
                "n" | "N" | "no" => {}
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
            println!("Do you want to download the following package?");
            println!("----------------------------------------------");
            println!("Name: {}", pkginfo.info.name);
            println!("Version: {}", pkginfo.info.version);
            println!("Description: {}", pkginfo.info.description);
            println!("Source: {}", pkginfo.info.source);
            println!("[y/n]-----------------------------------------");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            let build_info = pkginfo.build.inst;

            match choice {
                "y" | "Y" | "yes" => {
                    println!("The following command(s) or script(s) will be run:");
                    println!("---------------------------------------------------");
                    println!("{}", build_info);
                    println!("[ENTER]--------------------------------------------");

                    let mut wait = String::new();
                    io::stdin().read_line(&mut wait).unwrap();

                    let install = Command::new("sh")
                        .arg("-c")
                        .arg(&build_info)
                        .status()
                        .expect("Failed to install");

                    println!("Finish with code {install}");
                }
                "n" | "N" | "no" => {}
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
