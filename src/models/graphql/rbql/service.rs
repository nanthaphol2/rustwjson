use super::{
    super::rawdata::service::{
        get_rawdata_by_id, 
    },
    model::*,
    utils::*,
};


pub async fn get_rbql(id: u32, channel: RbChannel) -> RbResponse {
    let rawdata = get_rawdata_by_id(id, channel).await;

    if rawdata.id.is_none() {
        return RbResponse {
            success: false,
            error: Some("Not found"),
            data: None,
        };
    }

    RbResponse {
        success: true,
        error: None,
        data: Some(map_rbql(rawdata)),
    }
}

