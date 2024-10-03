use diesel::expression::AsExpression;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::ImageTable;
use diesel::deserialize::FromSqlRow;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = ImageTable)]
pub struct Image {
    pub id: i32,

    #[diesel(column_name = "imageType")]
    #[diesel(serialize_as = i32, deserialize_as = i32)]
    pub image_type: ImageType,

    /// image path in the local storage
    pub path: String,

    #[diesel(column_name = blogpostId)]
    pub blogpost_id: i32
}

#[derive(Debug, PartialEq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = diesel::sql_types::Integer)]
pub enum ImageType {
    /// avatar of the user
    Avatar = 0,

    /// image of the blogpost itself
    BlogPost = 1
}

impl From<i32> for ImageType {
    fn from(value: i32) -> ImageType {
        match value {
            0 => ImageType::Avatar,
            _ => ImageType::BlogPost
        }
    }
}

impl Into<i32> for ImageType {
    fn into(self) -> i32 {
        self as i32
    }
}
