use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    id: String,
    title: String,
}

#[derive(Deserialize, Debug)]
struct Listing {
    data: ListingData,
}

#[derive(Deserialize, Debug)]
struct ListingData {
    children: Vec<PostWrapper>,
}

#[derive(Deserialize, Debug)]
struct PostWrapper {
    data: Post,
}

pub struct RedditConfig<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

pub struct RedditBuilder<'a> {
    config: &'a RedditConfig<'a>,
    client: Client,
    token: Option<String>,
}
impl<'a> RedditBuilder<'a> {
    pub fn new(config: &'a RedditConfig<'a>) -> Self {
        let client = Client::builder()
            .user_agent("Liker/0.1 by Background-Log6333")
            .build()
            .expect("Failed to build reqwest client");
        RedditBuilder {
            config,
            client,
            token: None,
        }
    }

    pub async fn try_authenticate(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let params = [
            ("grant_type", "password"),
            ("username", self.config.username),
            ("password", self.config.password),
        ];

        println!("Authenticating...");

        let response = self
            .client
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(self.config.client_id, Some(self.config.client_secret))
            .form(&params)
            .send()
            .await;
        let response_code = response.as_ref().unwrap().status();
        println!("Response code: {:?}", response_code);
        let response = response.unwrap().json::<serde_json::Value>().await.unwrap();

        match response.get("error") {
            Some(err) => {
                println!("Authentication error: {:?}", err);
                return Err("Failed to authenticate".into());
            }
            None => {
                if let Some(token) = response.get("access_token").and_then(|t| t.as_str()) {
                    self.token = Some(token.to_string());
                } else {
                    println!("No access token found in the response.");
                    return Err("Failed to authenticate".into());
                }
            }
        }

        Ok(self)
    }

    pub fn build(self) -> Reddit {
        Reddit {
            client: self.client,
            token: self.token.unwrap_or_default(),
        }
    }
}

pub struct Reddit {
    client: Client,
    token: String,
}

impl Reddit {
    /// Fetch the timeline first post from a specific subreddit.
    pub async fn fetch_first_post_from_subreddit(
        &self,
        subreddit: &str,
    ) -> Result<Post, Box<dyn std::error::Error>> {
        let url = format!("https://oauth.reddit.com/r/{}/new", subreddit);

        let timeline_response = self.client.get(url).bearer_auth(&self.token).send().await?;

        let timeline_json: Listing = timeline_response.json().await?;

        if let Some(first_post) = timeline_json.data.children.first() {
            Ok(first_post.data.clone())
        } else {
            Err("No posts found".into())
        }
    }

    /// Like a post
    pub async fn like_post(&self, post: &Post) -> Result<(), Box<dyn std::error::Error>> {
        let post_id = &post.id;
        println!("Liking post: {}", post.title);

        let like_response = self
            .client
            .post(format!(
                "https://oauth.reddit.com/api/vote?id=t3_{}",
                post_id
            ))
            .bearer_auth(&self.token)
            .form(&[("dir", "1"), ("id", &format!("t3_{}", post_id))])
            .send()
            .await?;

        if like_response.status().is_success() {
            println!("Successfully liked the post!");
            Ok(())
        } else {
            println!("Failed to like the post.");
            Err("Failed to like the post".into())
        }
    }
}
