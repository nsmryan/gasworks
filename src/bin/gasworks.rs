extern crate memmap;
extern crate csv;
extern crate serde;
#[cfg(feature = "profile")]extern crate flame;
extern crate gasworks;
extern crate crossbeam_channel;

use std::thread;
use std::io::{Cursor, Read};
use std::fs::File;

#[macro_use] extern crate quicli;
use quicli::prelude::*;

use memmap::{ MmapOptions };

use crossbeam_channel as channel;
use crossbeam_channel::{Receiver, Sender};

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


main!(|args: Cli, log_level : verbosity| {
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
    
    // MMap file
    //let file = File::open(args.infile)?;
    //let mmap = unsafe { MmapOptions::new().map(&file)? };
    //let length = mmap.len();
    //let mut bytes = Cursor::new(mmap);

    let packet : LayoutPacketDef = vn200_tlm;

    // Write CSV header
    layoutpacket_csvheader(&packet, &mut writer);

    let num_bytes = packet.num_bytes() as usize;
    println!("num_bytes = {}", num_bytes);

    let num_names = packet.names().len();

    let mut record : Vec<String> = Vec::with_capacity(num_names);
    for _ in 0..record.capacity() {
        record.push(String::with_capacity(64));
    }

    let maybe_located = packet.locate();
    let loc_layout = maybe_located.unwrap();

    let num_threads = 10;

    #[cfg(feature = "profile")] flame::start("main loop");

    // set up thread system
    let mut threads = Vec::new();
    let mut senders_locs: Vec<Sender<&[u8]>> = Vec::new();
    let mut receivers_points: Vec<Receiver<Vec<Point>>>   = Vec::new();

    for _ in 0..num_threads {
        let (send_loc, receive_loc) = channel::bounded(10);
        senders_locs.push(send_loc);

        let (send_points, receive_points) = channel::bounded(10);
        receivers_points.push(receive_points);

        let loc_layout = loc_layout.clone();
        threads.push(thread::spawn(move || {
            while let Some(layout_bytes) = receive_loc.recv() {
                let points = decode_loc_layout(&loc_layout, &mut Cursor::new(layout_bytes));
                send_points.send(points);
            }
        }));
    }

    threads.push(thread::spawn(move || {
        let mut record : Vec<String> = Vec::with_capacity(num_names);
        for _ in 0..record.capacity() {
            record.push(String::with_capacity(64));
        }

        let mut index = 0;

        while let Some(points) = receivers_points[index].recv() {
            points.iter()
                  .zip(record.iter_mut())
                  .map(|(point, csv_line)| {
                    csv_line.clear();
                    csv_line.push_str(&format!("{}", point.val));
            }).collect::<()>();

            writer.write_record(&record).unwrap();
            index = (index + 1) % num_threads;
        }
    }));

    // read whole file
    let mut byte_vec: Vec<u8> = Vec::new();
    File::open(args.infile).unwrap().read_to_end(&mut byte_vec).unwrap();
    let length = byte_vec.len() as usize;

    let mut index = 0;
    // Decode file and write out CSV
    let mut position : usize = 0;
    while (position + num_bytes) < length as usize {
        {
            let layout_bytes: &[u8] =
                &byte_vec[position .. (position + num_bytes as usize)];

            senders_locs[index].send(layout_bytes);

            index = (index + 1) % num_threads;
        }

        // advance cursor to next structure
        position += num_bytes as usize;
    }
    #[cfg(feature = "profile")] flame::end("main loop");

    for thread in threads {
        thread.join();
    }

    #[cfg(feature = "profile")]
    flame::dump_html(&mut File::create("flame-gasworks.html").unwrap()).unwrap();
});

