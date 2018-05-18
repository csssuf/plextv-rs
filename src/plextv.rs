use mediacontainer::*;
use util::*;
use pms::PlexMediaServer;

use chrono::{DateTime, Utc};
use failure::Error;
use reqwest::Client;
use reqwest::header::Headers;
use serde_xml_rs;

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

/// Client for the plex.tv API.
#[derive(Debug, Clone)]
pub struct PlexTV {
    client: Client,
    headers: Headers,
    user: PlexTVUser,
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
    ) -> Result<PlexTV, Error> {
        let client = Client::new()?;

        let mut product_headers = Headers::with_capacity(4);
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

        product_headers.set(XPlexToken(res_struct.user.clone().authentication_token));

        Ok(PlexTV {
            client: client,
            headers: product_headers,
            user: res_struct.user,
        })
    }

    #[inline]
    pub fn id(&self) -> u64 {
        self.user.id
    }

    #[inline]
    pub fn uuid(&self) -> &str {
        &self.user.uuid
    }

    #[inline]
    pub fn email(&self) -> &str {
        &self.user.email
    }

    #[inline]
    pub fn joined_at(&self) -> DateTime<Utc> {
        self.user.joined_at
    }

    #[inline]
    pub fn username(&self) -> &str {
        &self.user.username
    }

    #[inline]
    pub fn title(&self) -> &str {
        &self.user.title
    }

    #[inline]
    pub fn thumb(&self) -> &str {
        &self.user.thumb
    }

    #[inline]
    pub fn has_password(&self) -> bool {
        self.user.has_password
    }

    #[inline]
    pub fn authentication_token(&self) -> &str {
        &self.user.authentication_token
    }

    #[inline]
    pub fn roles(&self) -> Vec<&str> {
        self.user
            .roles
            .roles
            .iter()
            .map(|x| x.as_ref())
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn entitlements(&self) -> Vec<&str> {
        self.user
            .entitlements
            .iter()
            .map(|x| x.as_ref())
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn confirmed_at(&self) -> DateTime<Utc> {
        self.user.confirmed_at
    }

    #[inline]
    pub fn forum_id(&self) -> u64 {
        self.user.forum_id
    }

    #[inline]
    pub fn remember_me(&self) -> bool {
        self.user.remember_me
    }

    pub fn servers(&mut self, _include_dead: bool) -> Result<Vec<PlexMediaServer>, Error> {
        let res = self.client
            .get("https://plex.tv/pms/servers.xml")?
            .headers(self.headers.clone())
            .send()?;

        // TODO(csssuf): This is a lazy way of dealing with the fact that error-chain errors aren't
        // Sync and so don't compose well with failure and I'm not interested in shaving *that*
        // particular yak right now.
        // Fix this when serde-xml-rs switches to failure or error-chain comes up with something
        // for interop with failure or something.
        let res_struct: MediaContainer = match serde_xml_rs::from_reader(res) {
            Ok(s) => s,
            Err(e) => bail!("deserialization error: {}", e),
        };
        let mut out = Vec::new();

        for server in res_struct.servers {
            out.push(PlexMediaServer::new(
                self.client.clone(),
                self.headers.clone(),
                server,
            ));
        }

        Ok(out)
    }
}
