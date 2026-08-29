use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Remove a package
    #[arg(short = 'r', long = "remove", value_name = "PACKAGE")]
    pub pkgtd: Option<String>,

    /// Specifies the package to install (format: category/package e.g. editor/vim)
    #[arg(short = 'i', long = "install", value_name = "PACKAGE")]
    pub package: Option<String>,

    #[arg(short = 'l', long = "list", value_name = "PATH")]
    pub list: Option<String>,
}
