use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use url::{ParseError, Position, Url};


async fn get_base_url(url: &Url, doc: &Document) -> Result<Url, ParseError> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);

    base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)
}

async fn check_link(url: &Url) -> bool {
    let res = reqwest::get(url.as_ref()).await;
    let mut final_check = false;

    match res {
        Ok(response) => {
            final_check = response.status() != StatusCode::NOT_FOUND;
        },
        Err(_err) => {
            final_check = false;
        }
    }

    final_check
}

async fn parse_url(url: &str) -> String {
    let url = Url::parse("https://www.rust-lang.org/en-US/");

    let mut final_parsed: String = String::from("");

    match url {
        Ok(url) => {
            let res = reqwest::get(url.as_ref()).await;
            match res {
                Ok(url_res) => {
                    let final_res = url_res.text().await;
                    match final_res {
                        Ok(res_string) => {
                            final_parsed = res_string;
                        },
                        Err(_err) => {}
                    }
                },
                Err(_err) => {}
            }
        },
        Err(_err) => {}
    }

    final_parsed
}

#[tokio::main]
async fn main() {
    let init_url = "https://www.rust-lang.org/en-US/";

    let url = Url::parse(&init_url);
    match url {
        Ok(res_url) => {
            let res = parse_url(&String::from(init_url)).await;
            let res = res.as_str();

            let document = Document::from(res);
            let base_url = get_base_url(&res_url, &document).await;

            match base_url {
                Ok(base_url) => {
                    let base_parser = Url::options().base_url(Some(&base_url));
                    let links: HashSet<Url> = document
                        .find(Name("a"))
                        .filter_map(|n| n.attr("href"))
                        .filter_map(|link| base_parser.parse(link).ok())
                        .collect();
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
                            Ok(res) => {

                            },
                            Err(join_err) => {

                            }
                        }
                    }
                },
                Err(_err) => {}
            }
        },
        Err(parse_err) => {}
    }

}
