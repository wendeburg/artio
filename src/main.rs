use clap::{Parser, Subcommand};
use artio::{cmd::new::new_package, PackageCategories, VCSOptions};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about="Creates a new package")]
    New {
        #[arg(name="path", help="Set the path where the package directory will be created. If the directory already exists files will be put inside this directory")]
        path: String,

        #[arg(short, long, name="name", help="Set the package name. Defaults to directory name")]
        name: Option<String>,

        #[arg(short, long, name="category", value_enum, default_value_t=PackageCategories::Application, help="Set the package category.")]
        category: PackageCategories,

        #[arg(long, name="vcs", value_enum, default_value_t=VCSOptions::Git, help="Set the vcs to initialize")]
        vcs: VCSOptions
    },
}

fn main() {
    let args = Arguments::parse();

    match args.command {
        Some(Commands::New { path, name, category, vcs}) => new_package(&path, name, category, vcs),
        None => todo!(),
    }
}
