extern crate serde_json;

use error::*;
use util::*;

use chrono::{DateTime, Utc};
use reqwest::Client;
use reqwest::header::Headers;

/// Client for the plex.tv API.
#[derive(Debug, Clone)]
pub struct PlexTV {
    client: Client,
    headers: Headers,
    user: PlexTVUser,
}

#[derive(Debug, Clone)]
#[derive(Deserialize)]
struct PlexTVAuthResponse {
    user: PlexTVUser,
}

#[derive(Debug, Clone)]
#[derive(Deserialize)]
struct PlexTVUser {
    id: u64,
    uuid: String,
    email: String,
    joined_at: DateTime<Utc>,
    username: String,
    title: String,
    thumb: String,
    #[serde(rename = "hasPassword")] has_password: bool,
    authentication_token: String,
    subscription: PlexSubscription,
    roles: PlexRoles,
    entitlements: Vec<String>,
    #[serde(rename = "confirmedAt")] confirmed_at: DateTime<Utc>,
    #[serde(rename = "forumId")] forum_id: u64,
    #[serde(rename = "rememberMe")] remember_me: bool,
}

#[derive(Debug, Clone)]
#[derive(Deserialize)]
struct PlexSubscription {
    active: bool,
    status: String,
    plan: String,
    features: Vec<String>,
}

#[derive(Debug, Clone)]
#[derive(Deserialize)]
struct PlexRoles {
    roles: Vec<String>,
}

impl PlexTV {
    /// Create a new client for the plex.tv API.
    ///
    /// Of particular note are the `product`, `version`, and `identifier` arguments. The Plex API
    /// requires clients to set several headers containing metadata about themselves. `product`
    /// should contain the name of the product consuming the API; `version` should contain the
    /// version of the product, and `identifier` should be some sort of unique ID that identifies a
    /// specific instance of a client.
    pub fn new(
        product: String,
        version: String,
        identifier: String,
        username: String,
        password: String,
    ) -> Result<PlexTV> {
        let client = Client::new()?;

        let mut product_headers = Headers::with_capacity(3);
        product_headers.set(XPlexProduct(product));
        product_headers.set(XPlexVersion(version));
        product_headers.set(XPlexClientIdentifier(identifier));

        let mut res = client
            .post("https://plex.tv/users/sign_in.json")?
            .headers(product_headers.clone())
            .body(format!(
                "user[login]={}&user[password]={}",
                username,
                password
            ))
            .send()?;

        let res_struct: PlexTVAuthResponse = res.json()?;
        Ok(PlexTV {
            client: client,
            headers: product_headers,
            user: res_struct.user,
        })
    }
}
