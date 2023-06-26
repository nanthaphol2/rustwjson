pub mod rbql;
pub mod rawdata;

use async_graphql::{ 
    Schema,
    MergedObject, 
    EmptyMutation, 
    EmptySubscription,
};

use rawdata::RawDataQueryRoot;
use rbql::RbqlQueryRoot;

#[derive(MergedObject, Default)]
pub struct Query(RawDataQueryRoot, RbqlQueryRoot);

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(
        Query::default(),
        EmptyMutation,
        EmptySubscription
    )
    .limit_depth(8)
    .finish()
}