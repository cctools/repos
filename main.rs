use hyper::{Client, Uri};
use std::env;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let url: Uri = "http://httpbin.org/response-headers?foo=bar"
        .parse()
        .unwrap();

    match env::var("GITHUB_ACCESS_TOKEN") {
        Ok(_) => println!("Github token found"),
        Err(_) => println!("Github token not found"),
    }

    match env::var("GITLAB_ACCESS_TOKEN") {
        Ok(_) => println!("Gitlab token found"),
        Err(_) => println!("Gitlab token not found"),
    }

    match client.get(url).await {
        Ok(res) => println!("Response: {}", res.status()),
        Err(err) => println!("Error: {}", err),
    }
}
