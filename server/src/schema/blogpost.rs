use diesel::table;

table! {
    blogpost (id) {
        id -> Int4,
        text -> Varchar,
        username -> Varchar,
        dateOfPublication -> Date,
        image -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>
    }
}
