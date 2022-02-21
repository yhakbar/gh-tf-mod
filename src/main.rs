extern crate prettytable;

mod config;
mod gh;
mod tables;

use crate::config::Config;
use anyhow::Result;
use structopt::StructOpt;

use crate::tables::{print_module_table, print_modules_table};

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
        /// Provider to default to.
        #[structopt(short, long)]
        provider: Option<String>,
    },
    /// List available modules.
    #[structopt(name = "ls")]
    List {
        /// Module to inspect.
        module: Option<String>,
        /// Organization to list modules from.
        #[structopt(short, long)]
        org: Option<String>,
        /// Provider to list modules from.
        #[structopt(short, long)]
        provider: Option<String>,
        /// Print description of module.
        #[structopt(short, long)]
        description: bool,
        /// Print URL of module.
        #[structopt(short, long)]
        url: bool,
        /// JSON output.
        #[structopt(long)]
        json: bool,
        /// Don't use color in output.
        #[structopt(long)]
        no_color: bool,
        /// First 'f' modules to list. e.g. '-f 5' will list the first 5 modules.
        #[structopt(short, long)]
        first: Option<usize>,
        /// After 'a' cursor. e.g. '-a xyz=' will list modules after the xyz= cursor.
        #[structopt(short, long)]
        after: Option<String>,
        /// Show tags
        #[structopt(short, long)]
        tags: bool,
        /// Show releases
        #[structopt(short, long)]
        releases: bool,
    },
}

fn main() -> Result<()> {
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
            description,
            url,
            json,
            no_color,
            first,
            after,
            tags,
            releases,
        } => {
            let config = Config::load(&org, &provider);
            let list_org = &org.unwrap_or(config.org.unwrap());
            match module {
                Some(module) => {
                    let list_module_response =
                        gh::list_module(list_org.to_string(), provider, module, first, after)?;

                    if json {
                        println!("{}", serde_json::to_string(&list_module_response)?);
                    } else {
                        print_module_table(list_module_response, no_color, description, url);
                    }
                }
                None => {
                    let list_modules_response =
                        gh::list_modules(list_org.to_string(), provider, first, after).unwrap();
                    if json {
                        println!("{}", serde_json::to_string(&list_modules_response)?);
                    } else {
                        print_modules_table(
                            list_modules_response,
                            no_color,
                            description,
                            url,
                            tags,
                            releases,
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
