use chrono::NaiveDate;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::BlogPostTable;
use super::CreateBlogPostDTO;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = BlogPostTable)]
pub struct BlogPost {
    pub id: i32,

    /// not longer than 2000
    pub text: String,

    /// not longer than 128
    pub username: String,

    #[diesel(column_name = dateofpublication)]
    pub date_of_publication: NaiveDate,

    /// avatar uuid
    pub avatar: Option<String>,

    /// post image uuid
    #[diesel(column_name = postimage)]
    pub post_image: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = BlogPostTable)]
/// used when iserting a new post
pub struct NewPost {
    pub text: String,
    pub username: String,
    #[diesel(column_name = dateofpublication)]
    pub date_of_publication: NaiveDate,
    pub avatar: Option<String>,
    #[diesel(column_name = postimage)]
    pub post_image: Option<String>,
}

impl NewPost {
    pub fn from_create_blog_post_dto(dto: CreateBlogPostDTO, avatar: Option<String>, post_image: Option<String>) -> Self {
        NewPost {
            text: dto.text,
            username: dto.username,
            date_of_publication: dto.date_of_publication,
            avatar,
            post_image
        }
    }
}
