use anyhow::{Context, Result};
use diesel::{
    pg::PgConnection, ExpressionMethods, QueryDsl, RunQueryDsl
};
use crate::{
    models::BlogPost,
    schema::blogpost::blogpost::table as BlogpostTable,
};

/// number of blogposts in a feed page
const PAGE_SIZE: i64 = 5;

/// retuns a vector containing the returned rows, in this case a single post
pub fn create_blogpost(conn: &mut PgConnection, post: BlogPost) -> Result<Vec<BlogPost>> {
    diesel::insert_into(BlogpostTable)
        .values(&post)
        .load::<BlogPost>(conn)
        .map_err(anyhow::Error::from)
        .context("saving blogpost")
}

/// returns PAGE_SIZE number of results, ordered from the to the oldest blogpost
pub fn get_blogposts(conn: &mut PgConnection, page: u32) -> Result<Vec<BlogPost>> {
    use crate::schema::BlogPostTable::dsl::*;

    blogpost
        .order(dateOfPublication.desc())
        .limit(PAGE_SIZE)
        .offset(((page as i64)-1) * PAGE_SIZE)
        .load::<BlogPost>(conn)
        .map_err(anyhow::Error::from)
        .context("getting blogposts")
}
