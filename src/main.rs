
#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(unused_imports)]
use std::io::{Cursor, Read};

#[allow(unused_imports)]
use std::fs::File;

#[macro_use] extern crate quicli;
use quicli::prelude::*;

extern crate ron;
//use ron::ser::*;
use ron::de::*;

//#[macro_use]
extern crate serde;

extern crate gasworks;
use gasworks::*;
use gasworks::types::*;
use gasworks::csv::*;
//use gasworks::decode::*;

extern crate csv;


#[derive(Debug, StructOpt)]
struct Cli { format : String,

    infile : String,

    outfile : String,

    #[structopt(flatten)]
    verbosity : Verbosity,
}

/* Convienence functions for creating data definitions.  */
pub fn item(name : &str, typ : Prim) -> Item {
    Item::new(name.to_string(), typ)
}

pub fn u8_be()  -> Prim { Prim::Int(IntPrim::u8_be()) }
pub fn u8_le()  -> Prim { Prim::Int(IntPrim::u8_le()) }
pub fn u16_be() -> Prim { Prim::Int(IntPrim::u16_be()) }
pub fn u16_le() -> Prim { Prim::Int(IntPrim::u16_le()) }
pub fn u32_be() -> Prim { Prim::Int(IntPrim::u32_be()) }
pub fn u32_le() -> Prim { Prim::Int(IntPrim::u32_le()) }
pub fn u64_be() -> Prim { Prim::Int(IntPrim::u64_be()) }
pub fn u64_le() -> Prim { Prim::Int(IntPrim::u64_le()) }

pub fn i8_be()  -> Prim { Prim::Int(IntPrim::i8_be()) }
pub fn i8_le()  -> Prim { Prim::Int(IntPrim::i8_le()) }
pub fn i16_be() -> Prim { Prim::Int(IntPrim::i16_be()) }
pub fn i16_le() -> Prim { Prim::Int(IntPrim::i16_le()) }
pub fn i32_be() -> Prim { Prim::Int(IntPrim::i32_be()) }
pub fn i32_le() -> Prim { Prim::Int(IntPrim::i32_le()) }
pub fn i64_be() -> Prim { Prim::Int(IntPrim::i64_be()) }
pub fn i64_le() -> Prim { Prim::Int(IntPrim::i64_le()) }

pub fn f32_be() -> Prim { Prim::Float(FloatPrim::f32_be()) }
pub fn f32_le() -> Prim { Prim::Float(FloatPrim::f32_le()) }
pub fn f64_be() -> Prim { Prim::Float(FloatPrim::f64_be()) }
pub fn f64_le() -> Prim { Prim::Float(FloatPrim::f64_le()) }

pub fn val_u8  (value : u8)  -> Value { Value::U8(value)  }
pub fn val_u16 (value : u16) -> Value { Value::U16(value) }
pub fn val_u32 (value : u32) -> Value { Value::U32(value) }
pub fn val_u64 (value : u64) -> Value { Value::U64(value) }
pub fn val_i8  (value : i8)  -> Value { Value::I8(value)  }
pub fn val_i16 (value : i16) -> Value { Value::I16(value) }
pub fn val_i32 (value : i32) -> Value { Value::I32(value) }
pub fn val_i64 (value : i64) -> Value { Value::I64(value) }
pub fn val_f32 (value : f32) -> Value { Value::F32(value) }
pub fn val_f64 (value : f64) -> Value { Value::F64(value) }
pub fn val_enum(name : Name, value : i64) -> Value { Value::Enum(name, value) }

pub fn seq<T>(packets : Vec<Packet<T>>) -> Packet<T> {
    Packet::Seq(packets)
}

pub fn leaf<T>(item : T) -> Packet<T> {
    Packet::Leaf(item)
}

pub fn array_fixed<T>(size : usize, packet : Packet<T>) -> Packet<T> {
    Packet::Array(ArrSize::Fixed(size), Box::new(packet))
}

pub fn array_var<T>(name : Name, packet : Packet<T>) -> Packet<T> {
    Packet::Array(ArrSize::Var(name), Box::new(packet))
}


main!(|args: Cli, log_level : verbosity| {
    // Open output file
    let mut writer = csv::Writer::from_path(args.outfile).unwrap(); 

    // Open format file
    let layout_string = File::open(args.format).expect("could not read format file!");

    // read format file
    match from_reader(layout_string)
    {
        Ok(layout) => {
            // Open binary file
            let mut byte_vec = Vec::new();
            File::open(args.infile).unwrap().read_to_end(&mut byte_vec).unwrap();
            let mut bytes = Cursor::new(byte_vec.as_slice());

            // Write CSV header
            valuemap_csvheader(&layout, &mut writer);

            let packet : LayoutPacket
                = seq(vec!(leaf(item("uint8_t", u8_be())),
                           leaf(item("uint16_t", u16_be())),
                           leaf(item("uint32_t", u32_be()))));

            //let loc_packet : LocPacket = packet.locate();
            //let loc_layout = identify_locpacket(&loc_packet, &mut bytes);
            //let points = decode_loc_layout(&loc_layout, &mut bytes);
            //println!("printing loc packet");
            //println!("{:?}", points);
            //bytes.set_position(0);

            let map = decode_layoutpacket(&packet, &mut bytes);
            println!("printing layout packet");
            println!("{:?}", map);
            bytes.set_position(0);

            // set up decoding structures
            let located = layout.locate();
            println!("{:?}", located);
            // NOTE this assumes structures all have same size
            let num_bytes = located.num_bytes() as usize;

            // Decode file and write out CSV
            // NOTE assumes correctly formatted file!
            while bytes.position() < byte_vec.len() as u64 {
                let position = bytes.position() as usize;

                {
                    let layout_bytes = bytes.get_ref();

                    let layout_bytes = &layout_bytes[position .. (position + num_bytes)];

                    let points = decode_loc_layout(&located, &mut Cursor::new(layout_bytes));

                    let record : Vec<String> =
                      points.iter().map(|point| {point.val.to_string()}).collect();
                    writer.write_record(record).unwrap();
                }

                // advance cursor to next structure
                bytes.set_position((position + num_bytes) as u64);
            }

            // println!("{}", to_string_pretty(&layout, Default::default()).expect("couldn't serialize layout!"));

            // println!("{:?}", layout.names());
        },

        Err(e) => {
            println!("Failed to load cofig: {}", e);
        }
    };
});

