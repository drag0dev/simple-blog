use chrono::NaiveDate;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::blogpost::blogpost;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = blogpost)]
pub struct BlogPost {
    pub id: i32,
    pub text: String,
    pub username: String,
    #[diesel(column_name = dateOfPublication)]
    pub date_of_publication: NaiveDate,
    pub image: Option<String>,
    pub avatar: Option<String>
}
