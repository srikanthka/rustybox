use reqwest;
use std::error::Error;

pub async fn run(url: &str) -> Result<(), Box<dyn Error>>{
    println!(" Fetching: {}", url);

    let body = reqwest::get(url).await?.text().await?;

    println!(" Fetched content (first 500 chars):\n");
    println!("{}", &body[..body.len().min(500)]);

    Ok(())
}
