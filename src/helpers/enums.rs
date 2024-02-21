use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum UserSearchField {
    Id(Uuid),
    Email(String),
}
