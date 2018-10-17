extern crate memmap;
extern crate csv;
extern crate serde;
#[cfg(feature = "profile")]extern crate flame;
extern crate gasworks;
extern crate crossbeam_channel;
extern crate crossbeam;
//extern crate rayon;
//extern crate futures;
extern crate sorted_list;
extern crate revord;

//use std::thread;
use std::io::{Cursor, Read, Write};
use std::fs::File;
use std::collections::BinaryHeap;
//use std::sync::{Arc};

//use futures::future::*;

#[macro_use] extern crate quicli;
use quicli::prelude::*;

//use memmap::{ MmapOptions };

use crossbeam_channel as channel;
use crossbeam_channel::{Receiver, Sender};

use revord::RevOrd;

//use rayon::scope;

//use sorted_list::SortedList;

use gasworks::types::*;
use gasworks::csv::*;
use gasworks::decode::*;


#[derive(Debug, StructOpt)]
struct Cli {
    infile : String,

    outfile : String,

    #[structopt(short="t", long="threads", default_value="7")]
    num_threads: u16,

    #[structopt(short="p", long="packetqueue", default_value="20")]
    packet_queue_depth: u16,

    #[structopt(short="l", long="linequeue", default_value="100")]
    line_queue_depth: u16,

    #[structopt(short="s", long="single")]
    single_threaded: bool,

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

fn get_current_index<T: std::cmp::Ord>(queue: &BinaryHeap<(RevOrd<usize>, T)>) -> usize {
    let revord = queue.peek().unwrap();
    let current_index = &revord.0;

    return current_index.0;
}

main!(|args: Cli, log_level : verbosity| {
    // Open output file
    //let mut writer = csv::Writer::from_path(args.outfile).unwrap(); 
    let mut writer = File::create(args.outfile).unwrap();

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

    let num_threads: usize = args.num_threads as usize;

    let packet : LayoutPacketDef = vn200_tlm;

    // Write CSV header
    layoutpacket_csvheader(&packet, &mut writer);

    let num_names = packet.names().len();

    let maybe_located = packet.locate();
    let loc_layout = maybe_located.unwrap();

    // read whole file
    let byte_vec: Vec<u8>;

    {
        let mut byte_vec_mut: Vec<u8> = Vec::new();
        File::open(args.infile).unwrap().read_to_end(&mut byte_vec_mut).unwrap();
        byte_vec = byte_vec_mut;
    }

    let packet_stream = PacketStream::new(packet, &byte_vec);

    if args.single_threaded {
        let mut line = String::new();

        for packet in packet_stream {
            let points = decode_loc_layout(&loc_layout, &mut Cursor::new(packet));

            points_to_str(&points, &mut line);

            writer.write(line.as_bytes());
        }
    }
    else {
        let (send_line, receive_line) = channel::bounded(args.line_queue_depth as usize);

        let (pack_sender, pack_receiver) = channel::bounded(args.packet_queue_depth as usize);

        crossbeam::scope(|scope| {
            let mut join_handles = Vec::new();

            for _ in 0..num_threads {
                let join_handle = scope.spawn(|| {
                    while let Some(option_packet) = pack_receiver.recv() {
                        match option_packet {
                            Some((packet, index)) => {
                                let points = decode_loc_layout(&loc_layout, &mut Cursor::new(packet));

                                let mut records : Vec<String> = Vec::with_capacity(points.len());
                                for _ in 0..points.len() {
                                    records.push("".to_string());
                                }
                                let mut line = String::new();

                                points_to_str(&points, &mut line);

                                send_line.send(Some((line, index)));
                            },
                            None => break,
                        }
                    }
                });
                join_handles.push(join_handle);
            }

            scope.spawn(|| {
                let mut to_write = BinaryHeap::new();
                let mut next_index = 0;

                while let Some(option_line) = receive_line.recv() {
                    match option_line {
                        Some((line, index)) => {
                            to_write.push((RevOrd(index), line));

                            let ixs: Vec<usize> = to_write.iter().map(|(ix, _)| (*ix).0).collect();

                            // process stored records
                            while to_write.len() > 0 {
                                let current_index = get_current_index(&to_write);
                                if current_index == next_index {
                                    let (_, line) = to_write.pop().unwrap();
                                    writer.write(line.as_bytes());
                                    next_index += 1;
                                    writer.flush();
                                }
                                else {
                                    break;
                                }
                            }
                        },

                        None => break,
                    }
                }
            });

            let mut index = 0;
            for packet in packet_stream {
                pack_sender.send(Some((packet, index)));
                index += 1;
            }

            for _ in 0..num_threads {
                pack_sender.send(None);
            }

            for joiner in join_handles {
                joiner.join();
            }
            send_line.send(None);
        });
    }

    #[cfg(feature = "profile")]
    flame::dump_html(&mut File::create("flame-gasworks.html").unwrap()).unwrap();
});

