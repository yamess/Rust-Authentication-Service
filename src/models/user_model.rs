use diesel::{
    AsChangeset, Insertable, PgConnection, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users::dsl::*;

#[derive(Insertable, Queryable, Selectable, Deserialize, Serialize, AsChangeset, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct UserModel {
    pub id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub email: String,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub password: String,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub created_at: chrono::NaiveDateTime,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub updated_at: Option<chrono::NaiveDateTime>,
}
