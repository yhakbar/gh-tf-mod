extern crate prettytable;

mod config;
mod gh;
mod tables;

use crate::config::Config;
use structopt::StructOpt;

use crate::tables::print_modules_table;

/// GitHub CLI extension for managing local Terraform modules.
#[derive(StructOpt, Debug)]
#[structopt(name = "gh-tf-mod", author, about)]
enum Commands {
    /// Configure local defaults for things like the GitHub organization and the default provider.
    #[structopt(name = "config")]
    Config {
        /// Organization to default to.
        #[structopt(short, long)]
        org: Option<String>,
        // Provider to default to.
        #[structopt(short, long)]
        provider: Option<String>,
    },
    /// List avaialable modules.
    #[structopt(name = "ls")]
    List {
        /// Module to inspect.
        module: Option<String>,
        /// Organization to list modules from.
        #[structopt(short, long)]
        org: Option<String>,
        // Provider to list modules from.
        #[structopt(short, long)]
        provider: Option<String>,
        // JSON output.
        #[structopt(long)]
        json: bool,
        // Don't use color in output.
        #[structopt(long)]
        no_color: bool,
        // First 'f' modules to list. e.g. '-f 5' will list the first 5 modules.
        #[structopt(short, long)]
        first: Option<usize>,
        // After 'a' cursor. e.g. '-a GFa8P2pimQ7=' will list modules after the GFa8P2pimQ7 cursor.
        #[structopt(short, long)]
        after: Option<String>,
    },
}

fn main() -> std::io::Result<()> {
    let args = Commands::from_args();
    match args {
        Commands::Config { org, provider } => {
            let config = Config::new(org, provider);
            config.save()?;
        }
        Commands::List {
            module,
            org,
            provider,
            json,
            no_color,
            first,
            after,
        } => match module {
            Some(module) => {
                println!("One day, this will do something different with {}!", module)
            }
            None => {
                let config = Config::load(&org, &provider);
                let list_org = &org.unwrap_or(config.org.unwrap());
                let list_modules_response =
                    gh::list_modules(list_org.to_string(), first, after).unwrap();
                if json {
                    println!("{}", serde_json::to_string(&list_modules_response)?);
                } else {
                    print_modules_table(list_modules_response, no_color);
                }
            }
        },
    }
    Ok(())
}
