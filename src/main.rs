use clap::Parser;

mod mods;

use mods::cli::Args;
use mods::install::install_package;
use mods::list::list;
use mods::remove::remove_package;

fn main() {
    let args = Args::parse();

    match (args.pkgtd, args.package, args.list) {
        (Some(pkgtd), None, None) => {
            remove_package(&pkgtd);
        }
        (None, Some(pkg), None) => {
            install_package(&pkg);
        }
        (None, None, Some(path)) => {
            list(&path);
        }
        _ => {
            println!("no argument provided. --help for help");
        }
    }
}
