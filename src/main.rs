use clap::{Parser, ValueEnum};
use std::{env, process::Command};

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

fn main() {
    let args = Args::parse();

    match args.channel {
        Channels::Unstable => install_unstable(&args.package),
        Channels::Stable => install_stable(&args.package),
    }
}

fn install_unstable(pkg: &str) {
    let pkgurl = format!(
        "https://raw.githubusercontent.com/sre-repo/bin/refs/heads/main/unstable/{pkg}/package.toml"
    );

    println!("Installing {pkg} from {pkgurl}...");
    println!("Channel: Unstable");

    let home = env::var("HOME").expect("Somehow could not find HOME environment");
    let path = format!("{}/.srepkgs/{pkg}", home);

    let status = Command::new("wget")
        .arg("-q")
        .arg("-nc")
        .arg("-P")
        .arg(&path)
        .arg(&pkgurl)
        .status()
        .expect("Exit (Fail)");

    println!("Finish with code {status}");
}

fn install_stable(pkg: &str) {
    let pkgurl = format!(
        "https://raw.githubusercontent.com/sre-repo/bin/refs/heads/main/stable/{pkg}/package.toml"
    );

    println!("Installing {pkg} from {pkgurl}...");
    println!("Channel: Stable");

    let home = env::var("HOME").expect("Somehow could not find HOME environment");
    let path = format!("{}/.srepkgs/{pkg}", home);

    let status = Command::new("wget")
        .arg("-q")
        .arg("-nc")
        .arg("-P")
        .arg(&path)
        .arg(&pkgurl)
        .status()
        .expect("Exit (Fail)");

    println!("Finish with code {status}");
} // TODO: replace wget, actually install packages, handle dependencies, remove packages, handle system-wide packages
