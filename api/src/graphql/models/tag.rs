use crate::models::tag::Tag as TagModel;
use async_graphql::{SimpleObject, ID};

#[derive(SimpleObject, Clone)]
pub struct Tag {
    pub id: ID,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
}

impl From<TagModel> for Tag {
    fn from(value: TagModel) -> Self {
        Tag {
            id: value.id.into(),
            name: value.name,
            created_at: format!("{}", value.created_at),
            updated_at: format!("{}", value.updated_at),
            deleted: value.deleted,
        }
    }
}
