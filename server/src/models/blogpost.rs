use chrono::NaiveDate;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::BlogPostTable;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = BlogPostTable)]
pub struct BlogPost {
    pub id: i32,

    /// not longer than 2000
    pub text: String,

    /// not longer than 128
    pub username: String,

    #[diesel(column_name = dateOfPublication)]
    pub date_of_publication: NaiveDate,
}
