
#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::BTreeMap;

extern crate csv;

use std::fs::File;


use types::*;

// NOTE likely that performance can be improved here.
// profiling needed for stringifying values.
pub fn valuemap_csv(map : &ValueMap,
                    writer : &mut csv::Writer<File>)
{
    writer.write_record(map.values().iter().map(|value| {value.to_string()}));
}

pub fn valuemap_csvheader(layout : &Layout,
                         writer : &mut csv::Writer<File>)
{
    writer.write_record(layout.names().iter());
    //writer.write_record(map.keys().map(|value| {value.to_string()}));
}

