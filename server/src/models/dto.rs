use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateBlogPostDTO {
    /// not longer than 2000
    pub text: String,

    /// not longer than 128
    pub username: String,

    /// %Y-%M-%D
    pub date_of_publication: NaiveDate,
}
