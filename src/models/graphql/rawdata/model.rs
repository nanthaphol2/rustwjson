use serde::{ Serialize, Deserialize };
use serde_aux::prelude::*;

use async_graphql::*;

// use crate::models::graphql::rbql::model::RbChannel;

#[derive(Default)]
pub struct RawDataQuery;

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Rawdata {
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub id: Option<u32>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub qtype: Option<u32>,
    pub text: Option<String>,
    pub num_pages: Option<u16>,
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct MultipleRawdataResponse {
    pub success: bool,
    pub error: Option<&'static str>,
    pub data: Vec<Rawdata>
}

#[ComplexObject]
impl Rawdata {
}

