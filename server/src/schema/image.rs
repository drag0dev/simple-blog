use diesel::table;

table! {
    image (id) {
        id -> Int4,
        imageType -> Int4,
        path -> Varchar,
        blogpostId -> Int4
    }
}
