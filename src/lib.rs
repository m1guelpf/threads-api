#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use map_macro::hash_map;
use reqwest::ClientBuilder;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use types::{
	internal::{LikersResponse, ProfileResponse, Response, ThreadResponse, ThreadsResponse},
	PostResponse, Profile, ProfileDetail, Thread,
};

pub mod types;

/// Reverse engineered API client for Instagram's Threads app.
pub struct Threads {
	client: reqwest::Client,
}

impl Threads {
	/// Create a new instance of the API.
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn new() -> Self {
		let client = ClientBuilder::new()
			.user_agent("threads-api")
			.build()
			.unwrap();

		Self { client }
	}

	/// Get a user's profile.
	///
	/// # Arguments
	///
	/// * `user_id` - The user's ID.
	///
	/// # Errors
	///
	/// Returns an error if the API request fails.
	pub async fn profile(&self, user_id: &str) -> Result<Profile, Error> {
		let response = self
			.get::<Response<ProfileResponse>>("23996318473300828", json!({ "userID": user_id }))
			.await?;

		Ok(response.data.user_data.user)
	}

	/// Get a list of a user's posts.
	///
	/// # Arguments
	///
	/// * `user_id` - The user's ID.
	///
	/// # Errors
	///
	/// Returns an error if the API request fails.
	pub async fn posts(&self, user_id: &str) -> Result<Vec<Thread>, Error> {
		let response = self
			.get::<Response<ThreadsResponse>>("6232751443445612", json!({ "userID": user_id }))
			.await?;

		Ok(response
			.data
			.media_data
			.threads
			.into_iter()
			.map(Into::into)
			.collect())
	}

	/// Get a list of a user's replies.
	///
	/// # Arguments
	///
	/// * `user_id` - The user's ID.
	///
	/// # Errors
	///
	/// Returns an error if the API request fails.
	pub async fn replies(&self, user_id: &str) -> Result<Vec<Thread>, Error> {
		let response = self
			.get::<Response<ThreadsResponse>>("6307072669391286", json!({ "userID": user_id }))
			.await?;

		Ok(response
			.data
			.media_data
			.threads
			.into_iter()
			.map(Into::into)
			.collect())
	}

	/// Get a post's data.
	///
	/// # Arguments
	///
	/// * `post_id` - The post's ID.
	///
	/// # Errors
	///
	/// Returns an error if the API request fails.
	pub async fn post(&self, post_id: &str) -> Result<PostResponse, Error> {
		let response = self
			.get::<Response<Response<ThreadResponse>>>(
				"5587632691339264",
				json!({ "postID": post_id }),
			)
			.await?;

		Ok(PostResponse {
			post: response.data.data.containing_thread.into(),
			replies: response
				.data
				.data
				.reply_threads
				.into_iter()
				.map(Into::into)
				.collect(),
		})
	}

	/// Get a list of users who liked a post.
	///
	/// # Arguments
	///
	/// * `post_id` - The post's ID.
	///
	/// # Errors
	///
	/// Returns an error if the API request fails.
	pub async fn likes(&self, post_id: &str) -> Result<Vec<ProfileDetail>, Error> {
		let response = self
			.get::<Response<LikersResponse>>("9360915773983802", json!({ "mediaID": post_id }))
			.await?;

		Ok(response.data.likers.users)
	}

	async fn get<T: DeserializeOwned>(&self, doc_id: &str, variables: Value) -> Result<T, Error> {
		let response = self
			.client
			.post("https://www.threads.net/api/graphql")
			.header("x-ig-app-id", "238260118697367")
			.form(&hash_map! {
				"doc_id" => doc_id,
				"variables" => &variables.to_string(),
			})
			.send()
			.await?
			.error_for_status()?;

		Ok(response.json::<T>().await?)
	}
}

impl Default for Threads {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("{0}")]
	Reqwest(#[from] reqwest::Error),

	#[error("{0}")]
	Serde(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test(flavor = "multi_thread")]
	async fn can_get_zuck_profile() {
		let threads = Threads::default();
		let profile = threads.profile("314216").await.unwrap();

		assert_eq!(profile.username, "zuck");
		assert_eq!(profile.full_name, "Mark Zuckerberg");
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn can_get_zuck_posts() {
		let threads = Threads::default();
		let posts = threads.posts("314216").await.unwrap();

		let first_thread = posts.last().unwrap();

		assert_eq!(first_thread.id, "3138977881796614961");
		assert_eq!(
			first_thread.items[0].text,
			"Let's do this. Welcome to Threads. 🔥"
		);
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn can_get_zuck_replies() {
		let threads = Threads::default();
		let posts = threads.replies("314216").await.unwrap();

		let first_reply = posts.last().unwrap();

		assert_eq!(
			first_reply.items[1].text,
			"We're only in the opening moments of the first round here..."
		);
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn can_get_post_data() {
		let threads = Threads::default();
		let thread = threads.post("3138977881796614961").await.unwrap();

		assert_eq!(thread.post.id, "3138977881796614961");
		assert_eq!(
			thread.post.items[0].text,
			"Let's do this. Welcome to Threads. 🔥"
		);
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn can_get_post_likes() {
		let threads = Threads::default();
		let likers = threads.likes("3138977881796614961").await.unwrap();

		assert!(!likers.is_empty());
	}
}
