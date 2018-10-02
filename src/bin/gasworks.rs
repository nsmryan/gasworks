extern crate memmap;
extern crate csv;
extern crate serde;
#[cfg(feature = "profile")]extern crate flame;
extern crate gasworks;

#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(unused_imports)]
use std::io::{Cursor, Read};

#[allow(unused_imports)]
use std::fs::File;

#[macro_use] extern crate quicli;
use quicli::prelude::*;

use memmap::{ MmapOptions };

use gasworks::*;
use gasworks::types::*;
use gasworks::csv::*;
use gasworks::decode::*;


#[derive(Debug, StructOpt)]
struct Cli {
    infile : String,

    outfile : String,

    #[structopt(flatten)]
    verbosity : Verbosity,
}

/* Convienence functions for creating data definitions.  */
pub fn item(name : &str, typ : Prim) -> Item {
    Item::new(name.to_string(), typ)
}

pub fn u8_be(name : &str)  -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::u8_be()))) }
pub fn u8_le(name : &str)  -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::u8_le()))) }
pub fn u16_be(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::u16_be()))) }
pub fn u16_le(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::u16_le()))) }
pub fn u32_be(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::u32_be()))) }
pub fn u32_le(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::u32_le()))) }
pub fn u64_be(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::u64_be()))) }
pub fn u64_le(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::u64_le()))) }

pub fn i8_be(name : &str)  -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::i8_be()))) }
pub fn i8_le(name : &str)  -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::i8_le()))) }
pub fn i16_be(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::i16_be()))) }
pub fn i16_le(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::i16_le()))) }
pub fn i32_be(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::i32_be()))) }
pub fn i32_le(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::i32_le()))) }
pub fn i64_be(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::i64_be()))) }
pub fn i64_le(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Int(IntPrim::i64_le()))) }

pub fn f32_be(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Float(FloatPrim::f32_be()))) }
pub fn f32_le(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Float(FloatPrim::f32_le()))) }
pub fn f64_be(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Float(FloatPrim::f64_be()))) }
pub fn f64_le(name : &str) -> LayoutPacketDef { leaf(item(&name.to_string(), Prim::Float(FloatPrim::f64_le()))) }

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

pub fn seq<T>(name : Name, packets : Vec<PacketDef<T>>) -> PacketDef<T> {
    PacketDef::Seq(name, packets)
}

pub fn leaf<T>(item : T) -> PacketDef<T> {
    PacketDef::Leaf(item)
}

pub fn array_fixed<T>(name : Name, size : usize, packet : PacketDef<T>) -> PacketDef<T> {
    PacketDef::Array(name, ArrSize::Fixed(size), Box::new(packet))
}

pub fn array_var<T>(name : Name, var_name : Name, packet : PacketDef<T>) -> PacketDef<T> {
    PacketDef::Array(name, ArrSize::Var(var_name), Box::new(packet))
}


fn main_fn(args : Cli) -> Result<()>
{
    // Open output file
    let mut writer = csv::Writer::from_path(args.outfile).unwrap(); 

    let vn200_tlm : LayoutPacketDef = 
      seq("vn200".to_string(),
          vec!(seq("group1".to_string(),
                   vec!(u8_be("sync"),
                        u8_le("groups"),
                        u16_le("group1Flags"),
                        u16_le("group3Flags"),
                        u16_le("group4Flags"),
                        u16_le("group5Flags"),
                        u16_le("group6Flags")
                        )
               ),

               seq("group2".to_string(), vec!(u64_le("timeSinceStartup"),
                        u64_le("timeGPS"),
                        f32_le("yawPitchRoll[0]"),
                        f32_le("yawPitchRoll[1]"),
                        f32_le("yawPitchRoll[2]"),
                        f32_le("angularRate[0]"),
                        f32_le("angularRate[1]"),
                        f32_le("angularRate[2]"),
                        f64_le("position[0]"),
                        f64_le("position[1]"),
                        f64_le("position[2]"),
                        f32_le("velocity[0]"),
                        f32_le("velocity[1]"),
                        f32_le("velocity[2]"),
                        u16_le("insStatus")
                        )
               ),

               seq("group3".to_string(), vec!(f32_le("temperature"),
                        f32_le("pressure"),
                        u16_le("sensor_status"),
                        u8_le("nGpsSats"),
                        u8_le("gpsFix"),
                        f64_le("gpsPos[0]"),
                        f64_le("gpsPos[1]"),
                        f64_le("gpsPos[2]"),
                        f32_le("gpsUncert[0]"),
                        f32_le("gpsUncert[2]"),
                        f32_le("gpsUncert[2]"),
                        u16_le("vpeStatus"),
                        f32_le("insPosUncertainty"),
                        f32_le("insVelUncertainty"),
                        u16_le("crc16")
                        )
               ),

               u16_le("ccsds_crc16")
              )
          );
    

    // Open binary file
    let file = File::open(args.infile)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    let length = mmap.len();
    //File::open(args.infile).unwrap().read_to_end(&mut byte_vec).unwrap();
    let mut bytes = Cursor::new(mmap);

    let packet : LayoutPacketDef = vn200_tlm;

    // Write CSV header
    layoutpacket_csvheader(&packet, &mut writer);

    let num_bytes = packet.num_bytes();
    println!("num_bytes = {}", num_bytes);

    let names = packet.names();

    let mut record : Vec<String> = Vec::with_capacity(names.len());
    for _ in 0..record.capacity() {
        record.push(String::with_capacity(64));
    }

    let maybe_located = packet.locate();

    #[cfg(feature = "profile")] flame::start("main loop");

    // Decode file and write out CSV
    while (bytes.position() + num_bytes) < length as u64 {
        let position = bytes.position() as usize;

        {
            let layout_bytes = bytes.get_ref();

            let layout_bytes = &layout_bytes[position .. (position + num_bytes as usize)];

            match maybe_located {
                Some(ref loc_layout) => {
                    let points =
                        decode_loc_layout(loc_layout, &mut Cursor::new(layout_bytes))
                           .iter()
                           .zip(record.iter_mut())
                           .map(|(point, csv_line)|  {
                                csv_line.clear();
                                csv_line.push_str(&format!("{}", point.val));
                              })
                           .collect::<()>();
                                                             
                },

                None => {
                    #[cfg(feature = "profile")] flame::start("decode packet");
                    let points = decode_layoutpacket(&packet, &mut Cursor::new(layout_bytes));
                    #[cfg(feature = "profile")] flame::end("decode packet");

                    #[cfg(feature = "profile")] flame::start("create line");
                    points.values()
                          .iter()
                          .zip(record.iter_mut())
                          .map(|(value, csv_line)| { 
                              csv_line.clear();
                              csv_line.push_str(&format!("{}", value));
                          })
                          .collect::<()>();
                    #[cfg(feature = "profile")] flame::end("create line");
                },
            }

            #[cfg(feature = "profile")] flame::start("write line");
            writer.write_record(&record).unwrap();
            #[cfg(feature = "profile")] flame::end("write line");
        }

        // advance cursor to next structure
        bytes.set_position((position + num_bytes as usize) as u64);
    }
    #[cfg(feature = "profile")] flame::end("main loop");

    #[cfg(feature = "profile")]
    flame::dump_html(&mut File::create("flame-gasworks.html").unwrap()).unwrap();

    Ok(())
}

main!(|args: Cli, log_level : verbosity| {
    main_fn(args).unwrap();
});

