
#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(unused_imports)]
use std::io::{Cursor, Read};

#[macro_use] extern crate quicli;
use quicli::prelude::*;

extern crate packet_tool;
use packet_tool::*;
use packet_tool::types::*;
use packet_tool::csv::*;

extern crate csv;


#[derive(Debug, StructOpt)]
struct Cli {
    // infile : String,

    outfile : String,

    #[structopt(flatten)]
    verbosity : Verbosity,
}

main!(|args: Cli, log_level : verbosity| {
    let mut writer = csv::Writer::from_path(args.outfile).unwrap();

    let v = vec![0x12, 0x34, 0x56, 0x78,
                 0x12, 0x34, 0x56, 0x78,
                 0xAA
                ];
    let mut bytes = Cursor::new(v.as_slice());

    let prim_layout0 = Layout::Prim(Item::new("prim0".to_string(), Prim::Int(IntPrim::u8_be())));
    let prim_layout1 = Layout::Prim(Item::new("prim1".to_string(), Prim::Int(IntPrim::u16_be())));
    let prim_layout2 = Layout::Prim(Item::new("prim2".to_string(), Prim::Int(IntPrim::u8_be())));
    let layout = Layout::Seq(vec![prim_layout0, prim_layout1, prim_layout2]);

    let map = decode(&layout, &mut bytes);

    valuemap_csvheader(&map, &mut writer);
    valuemap_csv(&map, &mut writer);

});

