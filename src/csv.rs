
#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::BTreeMap;

extern crate csv;

use std::fs::File;
use std::io::Write;


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
                              writer : &mut File)
{
    let mut line = String::new();

    packet.names().iter().map(|s| {
        line.push_str(s);
        line.push_str(",");
    }).collect::<Vec<()>>();

    line.push_str("\n");

    writer.write(line.as_bytes());
}

pub fn points_to_str(points: &Vec<Point>, line: &mut String) {
    line.clear();
    for point in points {
        line.push_str(&format!("{}", point.val));
        line.push_str(",");
    }
    line.push_str("\n");
}

