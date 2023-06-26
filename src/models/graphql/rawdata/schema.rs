use async_graphql::Object;

use crate::models::graphql::rbql::model::RbChannel;

use super::{
    model::Rawdata,
    service::*,
};

#[derive(Default)]
pub struct RawDataQueryRoot;

#[Object]
impl RawDataQueryRoot {
    pub async fn rb(&self) -> RawDataQuery {
        RawDataQuery::default()
    }
}

#[derive(Default)]
pub struct RawDataQuery;

#[Object]
impl RawDataQuery {
    pub async fn by(
        &self,
        id: u32,
        #[graphql(default_with = "RbChannel::Channel1")] channel: RbChannel,
    ) -> Rawdata {
        get_rawdata_by_id(id, channel).await
    }
}
