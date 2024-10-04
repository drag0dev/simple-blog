use chrono::NaiveDate;
use serde::Deserialize;

pub const MAX_DATA_SIZE: usize = 2138;
pub const MAX_IMAGE_SIZE: usize = 2 * 1024 * 1024;
pub const TOTAL_PAYLOAD_SIZE: usize = MAX_DATA_SIZE + 2*MAX_IMAGE_SIZE;

/// text max len - 2000b
/// username max len - 128b
/// date max len - 10b
/// avatar max size - 2mb
/// post image max size - 2mb
#[derive(Debug, Deserialize)]
pub struct CreateBlogPostDTO {
    pub text: String,
    pub username: String,
    pub date_of_publication: NaiveDate,
}
