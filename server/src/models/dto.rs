use serde::{Deserialize, Serialize};

use super::BlogPost;

pub const MAX_TEXT_SIZE: usize = 2000;
pub const MAX_USERNAME_SIZE: usize = 128;
pub const MAX_IMAGE_SIZE: usize = 2 * 1024 * 1024;

/// text max len - 2000b
/// username max len - 128b
/// date max len - 10b
/// avatar max size - 2mb
/// post image max size - 2mb
#[derive(Debug, Deserialize)]
pub struct CreateBlogPostDTO {
    pub text: String,
    pub username: String,
    /// avatar image url
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FeedDTO {
    pub blogposts: Vec<BlogPost>
}

impl FeedDTO {
    pub fn new(blogposts: Vec<BlogPost>) -> Self {
        FeedDTO {
            blogposts
        }
    }
}
