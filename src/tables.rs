use diesel::prelude::*;


table! {
    users {
        id -> uuid,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}