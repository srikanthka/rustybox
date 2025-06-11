use serde::Deserialize;
use reqwest::Error;

#[derive(Debug, Deserialize)]
pub struct Post {
    userId: u32,
    id:u32,
    title:String,
    body:String,
}

pub async fn run() -> Result<(), Error> {
   let url = "https://jsonplaceholder.typicode.com/posts/1";
   println!("Fetching post from: {}", url);

   let response = reqwest::get(url).await?;
   let post: Post = response.json().await?;

   println!(" Post received:\n");
   println!("ID: {}", post.id);
   println!("Title:{}", post.title);
   println!("Body:\n{}", post.body);

   Ok(())
}
