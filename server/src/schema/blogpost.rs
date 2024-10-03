use diesel::table;

table! {
    blogpost (id) {
        id -> Int4,
        text -> Varchar,
        username -> Varchar,
        dateofpublication -> Date,
        avatar -> Nullable<VarChar>,
        postimage -> Nullable<VarChar>
    }
}
