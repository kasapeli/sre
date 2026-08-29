use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Remove a package. Format: category/package (e.g., editor/vim). Removing an entire category is also possible.
    #[arg(short = 'r', long = "remove", value_name = "PACKAGE")]
    pub pkgtd: Option<String>,

    /// Specifies the package to install. Format: category/package (e.g., editor/vim)
    #[arg(short = 'i', long = "install", value_name = "PACKAGE")]
    pub package: Option<String>,

    /// Lists current installed packages. Format: category/package, category, or . for root (e.g., editor/vim)
    #[arg(short = 'l', long = "list", value_name = "PATH")]
    pub list: Option<String>,
}
