use std::collections::HashSet;

use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
use url::{Url};
use clap::{Parser, Subcommand};


/// Check url for status code 404
async fn check_link(url: &Url) -> bool {
    // True if link works
    let final_check: bool;

    match reqwest::get(url.as_ref()).await {
        Ok(response) => {
            final_check = response.status() != StatusCode::NOT_FOUND;
        },
        Err(_err) => {
            final_check = false;
        }
    }

    final_check
}


/// Parse website by url as Document
async fn parse_document(url: &str) -> Document {
    let mut final_parsed: String = String::from("");

    match Url::parse(url) {
        Ok(url) => {
            match reqwest::get(url.as_ref()).await {
                Ok(url_res) => {
                    match url_res.text().await {
                        Ok(res_string) => { final_parsed = res_string; },
                        Err(_err) => {}
                    }
                },
                Err(_err) => {}
            }
        },
        Err(_err) => {}
    }

    Document::from(final_parsed.as_str())
}


/// Extract all links which can be found at Url
fn extract_links(doc: &Document, base_url: &Url) -> HashSet<Url> {
    let base_parser = Url::options().base_url(Some(&base_url));
    let links: HashSet<Url> = doc
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();

    links
}


/// Check if provided links work
async fn check_links(links: HashSet<Url>) {
    let mut tasks = vec![];

    for link in links {
        tasks.push(tokio::spawn(async move {
            if check_link(&link).await {
                println!("{} is OK", link);
            } else {
                println!("{} is Broken", link);
            }
        }));
    }

    for task in tasks {
        match task.await {
            Ok(_res) => {},
            Err(_join_err) => {}
        }
    }
}


/// Extract all links from Url string
async fn extract_links_from_url(url: &str) -> HashSet<Url> {
    let mut links: HashSet<Url> = Default::default();

    // parse str to Url
    match Url::parse(&url) {
        Ok(res_url) => {
            // read website as Document
            let document = parse_document(url).await;

            links = extract_links(&document, &res_url);
        },
        Err(_parse_err) => {}
    }

    links
}


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets target host
    #[arg(short, long, value_name = "HOST")]
    target: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
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


async fn process_cli(cli: &Cli) {
    // You can check the value provided by positional arguments, or option arguments
    cli_command(cli).await;
}


async fn cli_command(cli: &Cli) {
    match cli.target.as_ref() {
        Some(url) => {
            match cli.command {
                Some(Commands::Links {list, check}) => {
                    let links = extract_links_from_url(url.as_str()).await;

                    if check {
                        check_links(links).await;
                    } else if list {
                        for link in links {
                            println!("{}", link);
                        }
                    }
                },
                None => {}
            }
        },
        None => {}
    }
}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    process_cli(&cli).await;
}
