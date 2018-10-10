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

//use std::thread;
use std::io::{Cursor, Read};
use std::fs::File;
//use std::sync::{Arc};

//use futures::future::*;

#[macro_use] extern crate quicli;
use quicli::prelude::*;

//use memmap::{ MmapOptions };

use crossbeam_channel as channel;
use crossbeam_channel::{Receiver, Sender};

//use rayon::scope;

//use sorted_list::SortedList;

use gasworks::types::*;
use gasworks::csv::*;
use gasworks::decode::*;


#[derive(Debug, StructOpt)]
struct Cli {
    infile : String,

    outfile : String,

    #[structopt(short="t", long="threads", default_value="10")]
    num_threads: u16,

    #[structopt(short="p", long="packetqueue", default_value="20")]
    packet_queue_depth: u16,

    #[structopt(short="l", long="linequeue", default_value="20")]
    line_queue_depth: u16,

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

pub fn format_points(points: &Vec<Point>, records: &mut Vec<String>) {
    points.iter()
          .zip(records.iter_mut())
          .map(|(point, csv_line)| {
            csv_line.clear();
            let line = &format!("{}", point.val);
            csv_line.push_str(line);
    }).collect::<()>();
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

    let num_threads: usize = args.num_threads as usize;

    let packet : LayoutPacketDef = vn200_tlm;

    // Write CSV header
    layoutpacket_csvheader(&packet, &mut writer);

    let num_bytes = packet.num_bytes() as usize;
    let num_names = packet.names().len();

    let mut records : Vec<String> = Vec::with_capacity(num_names);
    for _ in 0..records.capacity() {
        records.push(String::with_capacity(64));
    }

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

    let (send_line, receive_line): (Sender<Option<Vec<String>>>, Receiver<Option<Vec<String>>>)
        = channel::bounded(args.line_queue_depth as usize);
    let (pack_sender, pack_receiver) =
        channel::bounded(args.packet_queue_depth as usize);

    crossbeam::scope(|scope| {
        for thread_index in 0..num_threads {
            scope.spawn(|| {
                //println!("processing thread spawned");
                while let Some(option_packet) = pack_receiver.recv() {
                    match option_packet {
                        Some(packet) => {
                            //println!("received packet");
                            let points = decode_loc_layout(&loc_layout, &mut Cursor::new(packet));

                            let mut records : Vec<String> = Vec::with_capacity(points.len());
                            for _ in 0..points.len() {
                                records.push("".to_string());
                            }

                            format_points(&points, &mut records);

                            send_line.send(Some(records));
                        },
                        None => break,
                    }
                }
                send_line.send(None);
                //println!("processing thread finished");
            });
        }

        scope.spawn(|| {
            //println!("writing thread spawned");
            let mut channel_index = 0;
            while let Some(option_records) = receive_line.recv() {
                match option_records {
                    Some(records) => {
                        writer.write_record(&mut records.iter());
                        channel_index = (channel_index + 1) % num_threads;
                    },

                    None => break,
                }
            }
            //println!("writing thread finished");
        });

        for packet in packet_stream {
            pack_sender.send(Some(packet));
            //println!("sent packet");
        }


        //println!("writing Nones");
        for thread_index in 0..num_threads {
            pack_sender.send(None);
        }
        //println!("wrote Nones");
    });

    #[cfg(feature = "profile")]
    flame::dump_html(&mut File::create("flame-gasworks.html").unwrap()).unwrap();
});

