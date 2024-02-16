use serde::Deserialize;
use crate::client::velog::VelogPost;

#[derive(Deserialize)]
pub(crate) struct VelogResponse {
    pub data: VelogTrendingPosts
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VelogTrendingPosts {
    pub trending_posts: Vec<VelogPost>
}