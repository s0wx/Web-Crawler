use clap::{Parser, Subcommand};
use crate::process_cli_command_links;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets target host
    #[arg(short, long, value_name = "HOST")]
    pub(crate) target: Option<String>,

    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}


#[derive(Subcommand)]
pub enum Commands {
    /// URL link extraction and checking
    Links {
        /// lists links of target
        #[arg(short, long)]
        list: bool,

        /// checks links of target
        #[arg(short, long)]
        check: bool,
    }
}


async fn cli_command(cli: &Cli) {
    match &cli.target {
        Some(check_target) => {
            let url = String::from(check_target);
            match cli.command {
                Some(Commands::Links {list, check}) => {
                    process_cli_command_links(&url, &list, &check).await;
                },
                None => {}
            }
        },
        None => {}
    }
}


pub async fn get_parser() {
    let cli = Cli::parse();
    cli_command(&cli).await;
}
