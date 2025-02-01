mod environment;
mod reddit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    environment::init_env();
    let config = environment::get_config();
    let client = reddit::RedditBuilder::new(config)
                            .try_authenticate()
                            .await?
                            .build();

    let post = client.fetch_first_post_from_subreddit("fujifilm").await?;
    println!("{:?}", post);
    client.like_post(&post).await?;
    Ok(())
}


