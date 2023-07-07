# Instagram Threads API

[![Crates.io](https://img.shields.io/crates/v/threads-api.svg)](https://crates.io/crates/threads-api)
[![Docs.rs](https://docs.rs/threads-api/badge.svg)](https://docs.rs/threads-api)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> Unofficial, Reverse-Engineered Rust client for Instagram's [Threads](https://threads.net).

## Usage

```rust
use threads_api::Threads;

let client = Threads::new();

let user = client.profile(user_id).await?;
let posts = client.posts(user_id).await?;
let posts = client.replies(user_id).await?;
```

## 📌 Roadmap

- [x] Get user profile
- [x] Get user posts
- [x] Get user replies
- [x] Get post replies
- [x] Get post likes
- [ ] Authentication
- [ ] Post a thread
- [ ] Post a reply
- [ ] Update profile details
- [ ] Follow a user
- [ ] Unfollow a user

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
