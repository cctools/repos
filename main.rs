use std::env;

use hyper::{Client, Body, Method, body::HttpBody as _};
use hyper::http::{Request};
use hyper_tls::HttpsConnector;
use tokio::io::{self, AsyncWriteExt as _};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // match env::var("GITHUB_ACCESS_TOKEN") {
    //     Ok(_) => println!("Github token found"),
    //     Err(_) => println!("Github token not found"),
    // }

    let github_token = env::var("GITHUB_ACCESS_TOKEN")?;
    let gitlab_token = env::var("GITLAB_ACCESS_TOKEN")?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let request = Request::builder()
        .method(Method::POST)
        .uri("https://api.github.com/api/v3/rate_limit")
        .header("User-Agent", "Cross-Company-Tools")
        .header("Authorization", format!("Bearer {}", github_token))
        .body(Body::from(""))?;

    let mut res = client.request(request).await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    while let Some(chunk) = res.body_mut().data().await {
        let chunk = chunk?;
        io::stdout()
            .write_all(&chunk)
            .await?
    }

    Ok(())
}
