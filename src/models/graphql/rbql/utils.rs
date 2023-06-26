use super::{
    model::*,
    super::rawdata::model::*
};

use cached::{ cached_key, TimedCache };

cached_key! {
    LENGTH: TimedCache<u32, Rbql> = TimedCache::with_lifespan(20);
    Key = { rawdata.id.unwrap_or(0) };
    fn map_rbql(rawdata: Rawdata) -> Rbql = {
        
        Rbql {
            id: rawdata.id.unwrap(),
            qtype: rawdata.qtype.unwrap(),
            text: rawdata.text.unwrap(),
            num_pages: rawdata.num_pages.unwrap(),
        }
    }
}
