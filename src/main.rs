extern crate prettytable;

mod config;
mod gh;
mod tables;

use crate::config::Config;
use anyhow::Result;
use structopt::StructOpt;

use crate::tables::{print_module_table, print_modules_table};

/// GitHub CLI extension for managing Terraform modules.
#[derive(StructOpt, Debug)]
#[structopt(name = "gh-tf-mod", author)]
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
    /// List information about modules.
    #[structopt(
        name = "ls",
        long_about = "
List information about modules.

If no arguments are provided, the default behavior is to list all modules.
If a module name is provided, the information for that module is displayed.

If more information is available for a paginated response, an `End Cursor` will be displayed.
To display values after that cursor, provide `End Cursor` as the value of the `-a|--after` argument.

Only repositories that follow the naming pattern `^terraform-([^-]+)-(.*)-module$` (e.g. terraform-s3-lambda-module) will be displayed.
Repos that do not match this pattern will be removed from results, and a `Hidden Repos` will be displayed to indicate how many were removed.

Change the paging size by changing the `-f|--first` argument.

Minimal information is displayed by default. Use flags like `-l|--long` to display more information.
"
    )]
    List {
        /// Module to inspect.
        module: Option<String>,
        /// Organization to list modules from.
        #[structopt(short, long)]
        org: Option<String>,
        /// Provider to list modules from.
        /// If missing, it must be prepended. e.g. `aws-s3`.
        #[structopt(short, long)]
        provider: Option<String>,
        /// Show descriptions.
        #[structopt(short, long)]
        description: bool,
        /// Show URLs.
        #[structopt(short, long)]
        url: bool,
        /// Print output in JSON format.
        #[structopt(short, long)]
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
        /// Show tags.
        #[structopt(short, long)]
        tags: bool,
        /// Show releases.
        #[structopt(short, long)]
        releases: bool,
        /// Activate all optional display flags
        #[structopt(short, long)]
        long: bool,
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
            long,
        } => {
            let config = Config::load(&org, &provider);
            match module {
                Some(module) => {
                    let list_module_response = gh::list_module(
                        config.org.unwrap().to_string(),
                        provider,
                        module,
                        first,
                        after,
                    )?;

                    if json {
                        println!("{}", serde_json::to_string(&list_module_response)?);
                    } else {
                        if long {
                            print_module_table(
                                list_module_response,
                                no_color,
                                true,
                                true,
                                true,
                                true,
                            );
                        } else {
                            print_module_table(
                                list_module_response,
                                no_color,
                                description,
                                url,
                                tags,
                                releases,
                            );
                        }
                    }
                }
                None => {
                    let list_modules_response =
                        gh::list_modules(config.org.unwrap().to_string(), provider, first, after)
                            .unwrap();
                    if json {
                        println!("{}", serde_json::to_string(&list_modules_response)?);
                    } else {
                        if long {
                            print_modules_table(
                                list_modules_response,
                                no_color,
                                true,
                                true,
                                true,
                                true,
                            );
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
    }
    Ok(())
}
