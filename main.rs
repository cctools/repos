use hyper::{Client, Uri};

#[tokio::main]
async fn main() {
    let client = Client::new();

    let url: Uri = "http://httpbin.org/response-headers?foo=bar"
        .parse()
        .unwrap();

    match client.get(url).await {
        Ok(res) => println!("Response: {}", res.status()),
        Err(err) => println!("Error: {}", err),
    }
}
