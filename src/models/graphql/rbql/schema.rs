use async_graphql::Object;


use super::{
    service::*,
    model::{
        RbResponse, 
        RbChannel
    },
};

#[derive(Default)]
pub struct RbqlQueryRoot;

#[Object]
impl RbqlQueryRoot {
    pub async fn rbql(&self) -> RbqlQuery {
        RbqlQuery::default()
    }
}

#[derive(Default)]
pub struct RbqlQuery;

#[Object]
/// Nhql (nHentai API)
/// 
/// Easier formatted data, ready to used out of the box
impl RbqlQuery {
    /// Get nHentai by ID (6 digits code)
    pub async fn by(
        &self, 
        id: u32,
        #[graphql(default_with = "RbChannel::Channel1")] channel: RbChannel
    ) -> RbResponse {
        get_rbql(id, channel).await
    }
}
