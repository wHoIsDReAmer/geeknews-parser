pub mod hada;
pub mod velog;

use async_trait::async_trait;

#[derive(Default)]
pub(crate) struct PublicPost {
    title: String,
    content: String,
    link: String,
    href: String
}

#[async_trait]
pub(crate) trait Parser {
    type Post;

    fn new(url: &str) -> Self;

    async fn last_post(&self) -> Self::Post;
    async fn ticker(self);
}

async fn send_webhook<T: AsRef<str> + reqwest::IntoUrl>(url: T, data: T) -> bool
    where reqwest::Body: From<T>{
    
    let client = reqwest::ClientBuilder::new()
        .build()
        .unwrap();

    let rst = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(data)
        .send()
        .await;

    rst.is_ok()
}