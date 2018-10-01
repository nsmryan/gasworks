
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
    writer.write_record(map.values().iter().map(|value| {value.to_string()})).unwrap();
}

pub fn layout_csvheader(layout : &Layout,
                         writer : &mut csv::Writer<File>)
{
    writer.write_record(layout.names().iter()).unwrap();
    //writer.write_record(map.keys().map(|value| {value.to_string()}));
}

pub fn layoutpacket_csvheader(packet : &LayoutPacketDef,
                              writer : &mut csv::Writer<File>)
{
    writer.write_record(packet.names().iter()).unwrap();
}

