extern crate bytes;
extern crate fnv;

#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::BTreeMap;
#[allow(unused_imports)]
use std::option;

use std::cmp;

#[allow(unused_imports)]
use self::bytes::{Bytes, Buf};

use types::*;


#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct LocLayout {
    pub loc_items: Vec<LocItem>,
}

impl LocLayout {
    pub fn new() -> LocLayout {
        LocLayout { loc_items: Vec::new() }
    }

    pub fn with_items(items: Vec<LocItem>) -> LocLayout {
        LocLayout { loc_items: items }
    }
}

impl NumBytes for LocLayout {
    fn num_bytes(&self) -> u64 {
        let mut num_bytes = 0;

        for loc_item in &self.loc_items {
            num_bytes = cmp::max(num_bytes, loc_item.loc + loc_item.typ.num_bytes());
        }

        num_bytes
    }
}

