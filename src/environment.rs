use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
}

pub fn init_env() {
    dotenv().ok();
}


pub fn get_config() -> Config {
    Config {
        client_id: std::env::var("REDDIT_CLIENT_ID").expect("REDDIT_CLIENT_ID not set"),
        client_secret: std::env::var("REDDIT_CLIENT_SECRET").expect("REDDIT_CLIENT_SECRET not set"),
        username: std::env::var("REDDIT_USERNAME").expect("REDDIT_USERNAME not set"),
        password: std::env::var("REDDIT_PASSWORD").expect("REDDIT_PASSWORD not set"),
    }
}




