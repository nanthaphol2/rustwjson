use super::{
    super::rbql::model::RbChannel,
    constant::*,
    model::*,
};

use tokio::fs;

use async_graphql::InputType;
use cached::{proc_macro::cached, TimedCache};


#[cached(
    type = "TimedCache<String, Option<Rawdata>>",
    create = "{ TimedCache::with_lifespan(3600 * 3) }",
    convert = r#"{ format!("{}-{}", channel.to_value(), id) }"#
)]
pub async fn internal_get_rawdata_by_id(id: u32, channel: RbChannel) -> Option<Rawdata> {

    if channel == RbChannel::Channel1 {
        match fs::read_to_string(format!("data/{}.json", id)).await {
            Ok(stringified_json) => {
                if let Ok(json) = serde_json::from_str::<Rawdata>(&stringified_json) {
                    return Some(json);
                }
            }
            Err(_) => {}
        }
    }

    None
}

pub async fn get_rawdata_by_id(id: u32, channel: RbChannel) -> Rawdata {
    if let Some(data) = internal_get_rawdata_by_id(id, channel).await {
        Rawdata {
            id: Some(id),
            qtype: data.qtype,
            text: data.text,
            num_pages: data.num_pages,
            
        }
    } else {
        match channel {
            RbChannel::Channel1 => EMPTY_RAWDATA_DATA,
            _ => EMPTY_RAWDATA_DATA,
        }
    }
}
