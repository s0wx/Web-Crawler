use std::string::String;

use crate::cli::{get_parser};
use crate::content::extract_links_from_url;
use crate::url_processing::check_urls;

mod cli;
mod content;
mod web_parsing;
mod url_processing;


async fn process_cli_command_links(url: &String, list: &bool, check: &bool) {
    let links = extract_links_from_url(url.as_str()).await;

    if *check {
        check_urls(links).await;
    } else if *list {
        for link in links {
            println!("{}", link);
        }
    }
}


#[tokio::main]
async fn main() {
    get_parser().await;
}
