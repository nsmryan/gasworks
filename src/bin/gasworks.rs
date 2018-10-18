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
use std::vec::Vec;
use std::iter::Filter;
use std::collections::BinaryHeap;
//use std::sync::{Arc};

//use futures::future::*;

#[macro_use] extern crate quicli;
use quicli::prelude::*;

use memmap::{ MmapOptions };

use crossbeam_channel as channel;
//use crossbeam_channel::{Receiver, Sender};

use revord::RevOrd;

use gasworks::*;
use gasworks::types::*;
use gasworks::csv::*;
use gasworks::decode::*;
use gasworks::value::*;


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

    #[structopt(short="i", long="items", default_value="")]
    items: String,

    #[structopt(flatten)]
    verbosity : Verbosity,
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
          vec!(seq("ccsds_pri".to_string(),
                   vec!(u16_le("packet_word"),
                        u16_le("seq"),
                        u16_le("size")
                        )
               ),

               seq("ccsds_sec".to_string(),
                   vec!(u16_le("flags"),
                        u32_le("time_seconds"),
                        u32_le("time_subseconds")
                        )
               ),

               seq("group1".to_string(),
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
    //let byte_vec = Vec::from_raw_parts(mmap.as_ptr(), length, length);

    let num_threads: usize = args.num_threads as usize;

    let packet : LayoutPacketDef = vn200_tlm;

    // read whole file
    let byte_vec: Vec<u8>;

    {
        let mut byte_vec_mut: Vec<u8> = Vec::new();
        File::open(args.infile).unwrap().read_to_end(&mut byte_vec_mut).unwrap();
        byte_vec = byte_vec_mut;
    }

    // create packet stream
    let mut loc_layout = packet.locate().unwrap();

    // filter items to parse, if provided on the command line
    if args.items.len() > 0 {
        let names: Vec<String> =
            args.items.split(",").map(|st| st.trim().to_string()).collect();

        loc_layout.loc_items
                  .retain(|item| {
                      names.contains(&item.name.last().unwrap())
                  });
    }

    // Write CSV header
    loclayout_csvheader(&loc_layout, &mut writer);

    let packet_stream = PacketStream::new(packet, &byte_vec);

    if args.single_threaded {
        let mut line = String::new();

        for packet in packet_stream {
            let points = decode_loc_layout(&loc_layout, &mut Cursor::new(packet));

            points_to_str(&points, &mut line);

            writer.write(line.as_bytes()).unwrap();
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

                            // process stored lines
                            while to_write.len() > 0 {
                                let current_index = get_current_index(&to_write);
                                if current_index == next_index {
                                    let (_, line) = to_write.pop().unwrap();
                                    writer.write(line.as_bytes()).unwrap();
                                    next_index += 1;
                                    writer.flush().unwrap();
                                }
                                else { // no lines to write
                                    break;
                                }
                            }
                        },

                        // if we receive None, end the task
                        None => break,
                    }
                }
            });

            // send packets to worker streams
            let mut index = 0;
            for packet in packet_stream {
                pack_sender.send(Some((packet, index)));
                index += 1;
            }

            // send Nones to clean up workers
            for _ in 0..num_threads {
                pack_sender.send(None);
            }

            // send None to writer thread to end it.
            send_line.send(None);
        });
    }

    #[cfg(feature = "profile")]
    flame::dump_html(&mut File::create("flame-gasworks.html").unwrap()).unwrap();
});

