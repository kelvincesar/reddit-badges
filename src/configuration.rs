use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The subreddit from which to fetch the first post
    #[arg(short, long, default_value = "fujifilm")]
    subreddit: String,

    /// Reddit client ID
    #[arg(long)]
    client_id: String,

    /// Reddit client secret
    #[arg(long)]
    client_secret: String,
        
    /// Reddit username
    #[arg(long)]
    username: String,

    /// Reddit password
    #[arg(long)]
    password: String,
}

impl Args {
    pub fn get_subreddit(&self) -> &str {
        &self.subreddit
    }
    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }
    pub fn get_client_secret(&self) -> &str {
        &self.client_secret
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
    
}


