use async_graphql::{ SimpleObject, ComplexObject, Enum };

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Enum, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RbChannel {
    Channel1 = 0,
    Channel2 = 1,
    Channel3 = 2
}

#[derive(Serialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Rbql {
    pub id: u32,
    pub text: String,
    pub qtype: u32,
    pub num_pages: u16,
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct MultipleRbResponse {
    pub success: bool,
    pub error: Option<&'static str>,
    pub data: Vec<RbResponse>
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct RbResponse {
    pub success: bool,
    pub error: Option<&'static str>,
    pub data: Option<Rbql>
}

#[ComplexObject]
impl Rbql {
}