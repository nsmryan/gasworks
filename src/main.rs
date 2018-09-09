
#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(unused_imports)]
use std::io::{Cursor, Read};

#[allow(unused_imports)]
use std::fs::File;

#[macro_use] extern crate quicli;
use quicli::prelude::*;

extern crate ron;
use ron::ser::*;
use ron::de::*;

#[macro_use]
extern crate serde;

extern crate packet_tool;
use packet_tool::*;
use packet_tool::types::*;
use packet_tool::csv::*;

extern crate csv;


#[derive(Debug, StructOpt)]
struct Cli {
    format : String,

    infile : String,

    outfile : String,

    #[structopt(flatten)]
    verbosity : Verbosity,
}

main!(|args: Cli, log_level : verbosity| {
    let mut writer = csv::Writer::from_path(args.outfile).unwrap();

    let layout_string = File::open(args.format).expect("could not read format file!");
    match from_reader(layout_string)
    {
        Ok(layout) => {
            let mut byte_vec = Vec::new();
            File::open(args.infile).unwrap().read_to_end(&mut byte_vec);
            let mut bytes = Cursor::new(byte_vec.as_slice());

            valuemap_csvheader(&layout, &mut writer);

            // NOTE assumes correctly formatted file!
            while bytes.position() < byte_vec.len() as u64 {
                let map = decode(&layout, &mut bytes);
                valuemap_csv(&map, &mut writer);
            }

            println!("{}", to_string_pretty(&layout, Default::default()).expect("couldn't serialize layout!"));

            println!("{:?}", layout.names());
        },

        Err(e) => {
            println!("Failed to load cofig: {}", e);
        }
    };
});

