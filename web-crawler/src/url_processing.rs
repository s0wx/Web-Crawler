use std::collections::{HashMap, HashSet};
use reqwest::StatusCode;
use url::Url;


/// Check url for status code 404
async fn check_url(url: &Url) -> bool {
    // True if url works
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


/// Check if provided urls work
pub async fn check_urls(urls: HashSet<Url>) {
    let mut tasks = vec![];

    for url in urls {
        tasks.push(tokio::spawn(async move {
            if check_url(&url).await {
                println!("{} is OK", url);
            } else {
                println!("{} is Broken", url);
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


pub fn get_query_params(url: &Url) -> HashMap<&str, &str> {
    let mut query_full: HashMap<&str, &str> = Default::default();

    match url.query() {
        Some(values) => {
            query_full = values.split("&")
                .map(|s| s.split_at(s.find("=").unwrap()))
                .map(|(key, val)| (key, &val[1..]))
                .collect();
        },
        None => {}
    }

    query_full
}
