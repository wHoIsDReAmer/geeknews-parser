use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
pub(crate) struct VelogPost {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) short_description: String,
    pub(crate) thumbnail: Option<String>,
    pub(crate) likes: usize,
    pub(crate) user: VelogUser,
    pub(crate) url_slug: String,
    pub(crate) released_at: String,
    pub(crate) updated_at: String,
    pub(crate) is_private: bool,
    pub(crate) comments_count: usize
}

#[derive(Deserialize, Default, Clone)]
pub(crate) struct VelogUser {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) profile: VelogProfile,
}

#[derive(Deserialize, Default, Clone)]
pub(crate) struct VelogProfile {
    pub(crate) id: String,
    pub(crate) thumbnail: Option<String>,
    pub(crate) display_name: String
}