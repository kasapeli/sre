use clap::Parser;

mod mods;

use mods::cli::Args;
use mods::install::install_package;
use mods::remove::remove_package;

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
