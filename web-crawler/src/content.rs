use std::collections::HashSet;
use select::document::Document;
use select::predicate::Name;
use url::Url;
use crate::web_parsing::parse_url_to_document;


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


/// Extract all links from Url string
pub async fn extract_links_from_url(url: &str) -> HashSet<Url> {
    let mut links: HashSet<Url> = Default::default();

    // parse str to Url
    match Url::parse(&url) {
        Ok(res_url) => {
            // read website as Document
            let document = parse_url_to_document(url).await;

            links = extract_links(&document, &res_url);
        },
        Err(_parse_err) => {}
    }

    links
}
