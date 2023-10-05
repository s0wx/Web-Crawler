use select::document::Document;
use url::Url;


/// Parse website by url as Document
pub async fn parse_url_to_document(url: &str) -> Document {
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
