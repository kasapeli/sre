use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Remove a package
    #[arg(long = "remove", value_name = "PACKAGE")]
    pub pkgtd: Option<String>,

    /// Specifies the package to install (format: category/package e.g. editor/vim)
    #[arg(long = "install", value_name = "PACKAGE")]
    pub package: Option<String>,

    #[arg(long = "list", value_name = "PATH")]
    pub list: Option<String>,
}
