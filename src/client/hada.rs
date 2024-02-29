use std::fmt::{Debug, Display};
use std::time::Duration;
use async_trait::async_trait;
use super::PublicPost;
use super::Parser;

#[derive(Default)]
pub struct HadaGeekNews {
    pub webhook_url: String,
    last_post: PublicPost
}

#[async_trait]
impl Parser for HadaGeekNews {
    type Post = PublicPost;

    fn new(url: &str) -> Self {
        HadaGeekNews {
            webhook_url: url.to_owned(),
            ..Default::default()
        }
    }

    async fn last_post(&self) -> Self::Post {
        let client = reqwest::ClientBuilder::new()
            .build()
            .unwrap();

        if let Ok(result) = client.get("https://news.hada.io/new")
            .send()
            .await {
            let body = result.text().await.unwrap_or("".into());
            if body.is_empty() {
                return PublicPost::default()
            }

            let topic = body.split("<div class=\'topic_row\'>").collect::<Vec<&str>>();

            let topic_title = topic[1].split("<div class=topictitle>").collect::<Vec<&str>>();
            let topic_title = topic_title[1].split("<h1>").collect::<Vec<&str>>();

            let topic_link = topic[1].split("<a href='").collect::<Vec<&str>>()[1];
            let topic_id = topic_title[1].split("<a href='").collect::<Vec<&str>>()[1];

            let topic_desc = topic[1].split(" breakall'>").collect::<Vec<&str>>()[1];

            let topic_link = topic_link.split("'").collect::<Vec<&str>>()[0];
            let topic_desc = topic_desc.split("<").collect::<Vec<&str>>()[0];
            let topic_id = topic_id.split("'").collect::<Vec<&str>>()[0];
            let topic_title = topic_title[1].split("</h1>").collect::<Vec<&str>>()[0];

            return PublicPost {
                title: topic_title.to_string(),
                content: topic_desc.to_string(),
                link: topic_id.to_string(),
                href: topic_link.to_string()
            }
        }

        PublicPost::default()
    }

    async fn ticker(mut self) {
        loop {
            let post = self.last_post().await;
            if post.title.eq("") {
                let _ = tokio::time::sleep(Duration::from_secs(1));
                continue
            }

            if self.last_post.title.ne(&post.title) {
                let webhook_url = self.webhook_url.clone();
                let body_data = make_webhook(&post.title, &post.content, &post.link, &post.href);
                super::send_webhook(webhook_url, body_data).await;
            }

            self.last_post = post;
            let _ = tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

fn make_webhook<T: AsRef<str>>(title: T, desc: T, url: T, og_url: T) -> String
    where T: Debug + Display {

    format!(r#"{{
"content": null,
"embeds": [
    {{
        "title": "{}",
        "description": {:?},
        "url": "https://news.hada.io/{}",
        "color": 5066061,
        "fields": [
            {{
                "name": ":link: 원본 링크",
                "value": "{}"
            }}
        ],
        "author": {{
            "name": "GeekNews",
            "icon_url": "https://news.hada.io/favicon.ico"
        }}
    }}
],
"attachments": []
}}"#, title, desc, url, og_url)
}