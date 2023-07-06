use serde::Deserialize;

pub(crate) mod internal {
    use super::{Media, Profile, ProfileDetail};

    #[derive(serde::Deserialize)]
    pub struct Response<T> {
        pub data: T,
    }

    #[derive(serde::Deserialize)]
    pub struct ProfileResponse {
        #[serde(rename = "userData")]
        pub user_data: UserFragment,
    }

    #[derive(serde::Deserialize)]
    pub struct ThreadsResponse {
        #[serde(rename = "mediaData")]
        pub media_data: ThreadsFragment,
    }

    #[derive(serde::Deserialize)]
    pub struct UserFragment {
        pub user: Profile,
    }

    #[derive(serde::Deserialize)]
    pub struct ThreadsFragment {
        pub threads: Vec<Thread>,
    }

    #[derive(serde::Deserialize)]
    pub struct Thread {
        pub id: String,
        pub thread_items: Vec<ThreadItem>,
    }

    #[derive(serde::Deserialize)]
    pub struct ThreadItem {
        pub post: Post,
    }

    #[derive(serde::Deserialize)]
    pub struct Post {
        pub user: ProfileDetail,
        #[serde(rename = "image_versions2")]
        pub images: ImageVersions,
        pub original_width: u32,
        pub original_height: u32,
        pub caption: Caption,
        pub taken_at: u64,
        pub like_count: u32,
    }

    #[derive(serde::Deserialize)]
    pub struct Caption {
        pub text: String,
    }

    #[derive(serde::Deserialize)]
    pub struct ImageVersions {
        pub candidates: Vec<Media>,
    }
}

/// Contains the minimum required information to display a profile.
#[derive(serde::Deserialize)]
pub struct ProfileDetail {
    pub profile_pic_url: String,
    pub username: String,
    pub is_verified: bool,
    #[serde(rename = "pk")]
    pub id: String,
}

/// Contains all the information available about a profile.
#[derive(Deserialize)]
pub struct Profile {
    #[serde(rename = "pk")]
    pub id: String,
    pub is_private: bool,
    pub profile_pic_url: String,
    pub username: String,
    pub is_verified: bool,
    pub biography: String,
    pub follower_count: u32,
    pub bio_links: Vec<Link>,
    pub full_name: String,
    pub hd_profile_pic_versions: Vec<Media>,
}

/// A link to an external website.
#[derive(Deserialize)]
pub struct Link {
    pub url: String,
}

/// A media item.
#[derive(Deserialize)]
pub struct Media {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

/// A thread of posts.
pub struct Thread {
    pub id: String,
    pub items: Vec<ThreadItem>,
}

impl From<internal::Thread> for Thread {
    fn from(value: internal::Thread) -> Self {
        Self {
            id: value.id,
            items: value
                .thread_items
                .into_iter()
                .map(|i| i.post.into())
                .collect(),
        }
    }
}

/// A post in a thread.
pub struct ThreadItem {
    pub likes: u32,
    pub text: String,
    pub published_at: u64,
    pub images: Vec<Media>,
    pub user: ProfileDetail,
}

impl From<internal::Post> for ThreadItem {
    fn from(thread: internal::Post) -> Self {
        Self {
            user: thread.user,
            likes: thread.like_count,
            text: thread.caption.text,
            published_at: thread.taken_at,
            images: thread.images.candidates,
        }
    }
}
