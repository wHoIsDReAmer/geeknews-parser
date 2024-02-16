mod post;
mod request;
mod response;

use std::fmt::{Debug, Display};
use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde::de::Error;

use crate::client::velog::post::VelogPost;
use crate::client::velog::request::{VelogGqlBody, VelogGqlVariables, VelogGqlVariablesInput};
use crate::client::velog::response::VelogResponse;

use super::Parser;

#[derive(Default)]
pub struct Velog {
    pub webhook_url: String,
    last_post: VelogPost
}

#[async_trait]
impl Parser for Velog {
    type Post = VelogPost;

    fn new(url: &str) -> Self {
        Velog {
            webhook_url: url.to_owned(),
            ..Default::default()
        }
    }

    async fn last_post(&self) -> Self::Post {
        let client = reqwest::ClientBuilder::new()
            .build()
            .unwrap();

        let gql_variable = VelogGqlVariables {
            limit: 20,
            offset: 20,
            timeframe: "day".to_owned()
        };

        let gql_variable_input = VelogGqlVariablesInput {
            input: gql_variable
        };

        let gql_body = VelogGqlBody {
            query: "\n    query trendingPosts($input: TrendingPostsInput!) {\n  trendingPosts(input: $input) {\n    id\n    title\n    short_description\n    thumbnail\n    likes\n    user {\n      id\n      username\n      profile {\n        id\n        thumbnail\n        display_name\n      }\n    }\n    url_slug\n    released_at\n    updated_at\n    is_private\n    comments_count\n  }\n}\n    "
                .to_owned(),
            variables: gql_variable_input
        };

        let serialized = serde_json::to_string(&gql_body).unwrap();

        let result = client.post("https://v3.velog.io/graphql")
            .header(CONTENT_TYPE, "application/json")
            .body(serialized)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let deserialized: Result<VelogResponse, serde_json::Error> = serde_json::from_str(&result);

        match deserialized {
            Ok(res) => {
                res.data.trending_posts[res.data.trending_posts.len() - 1].clone()
            },
            Err(err) => {
                println!("역직렬화 하는 도중 에러가 났습니다! {}", err.to_string());
                VelogPost::default()
            }
        }
    }

    async fn ticker(mut self) {
        loop {
            let last_post = self.last_post().await;
            if last_post.title.ne(&self.last_post.title) {
                let webhook_url = self.webhook_url.clone();
                let body_data = make_webhook(last_post.clone());
                super::send_webhook(webhook_url, body_data).await;
            }

            self.last_post = last_post;
            std::thread::sleep(std::time::Duration::from_secs(60))
        }
    }
}


fn make_webhook(velog_post: VelogPost) -> String {
    format!(r#"{{
"content": null,
"embeds": [
    {{
        "title": "{}",
        "description": {:?},
        "url": "https://velog.io/@{}/{}",
        "color": 9233266,
        "author": {{
            "name": "{}",
            "icon_url": "{}"
        }},
        "image": {{
            "url": "{}"
        }},
        "footer": {{
            "text": "❤ {}개의 좋아요 | 💬 {}개의 댓글"
        }}
    }}
],
"attachments": []
}}"#, velog_post.title,
        velog_post.short_description,
        velog_post.user.username,
        velog_post.url_slug,
        velog_post.user.profile.display_name,
        velog_post.user.profile.thumbnail
                .unwrap_or("https://images.velog.io/images/velog/profile/9aa07f66-5fcd-41f4-84f2-91d73afcec28/green%20favicon.png".into()),
        velog_post.thumbnail.unwrap_or("".into()),
        velog_post.likes,
        velog_post.comments_count,
        )
}