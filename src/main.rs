mod configuration;
mod reddit;

use clap::Parser;
use configuration::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = reddit::RedditConfig {
        client_id: args.get_client_id(),
        client_secret: args.get_client_secret(),
        username: args.get_username(),
        password: args.get_password(),
    };

    let client = reddit::RedditBuilder::new(&config)
        .try_authenticate()
        .await?
        .build();

    let post = client
        .fetch_first_post_from_subreddit(args.get_subreddit())
        .await?;
    println!("{:?}", post);
    client.like_post(&post).await?;
    Ok(())
}
