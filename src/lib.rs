#[cfg(feature = "profile")]extern crate flame;
extern crate bitreader;
extern crate bytes;
#[macro_use] extern crate serde;
extern crate byteorder;
extern crate ron;
extern crate fnv;

use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::BTreeMap;
#[allow(unused_imports)]
use std::iter::Iterator;

use fnv::FnvHashMap;

use bitreader::BitReader;

#[allow(unused_imports)]
use bytes::{Bytes, Buf};

#[allow(unused_imports)]
use byteorder::{LittleEndian, BigEndian, ByteOrder};



#[allow(unused_imports)]
use std::io::{Cursor, Read};

pub mod types;
use types::*;

pub mod decode;
use decode::*;

pub mod csv;


pub fn identify_locpacket(packet : &LocPacketDef, bytes : &mut Cursor<&[u8]>) -> LocLayout
{
    let locs = Vec::new();

    let mut loc_layout = LocLayout{ loc_items : locs};

    identify_locpacket_helper(packet, bytes, &mut loc_layout);

    loc_layout
}

fn identify_locpacket_helper(packet : &LocPacketDef, 
                             bytes : &mut Cursor<&[u8]>,
                             loc_layout : &mut LocLayout) 
{
    match packet {
        PacketDef::Seq(packets) => {
            for packet in packets {
                identify_locpacket_helper(packet, bytes, loc_layout);
            }
        },

        PacketDef::Subcom(item, subcom) => {
            // NOTE we are decoding items here and throwing them away. the assumption is that
            // we don't decode many items, and don't need to keep our work.
            // we re-decode items even if they are used in other iterations of this loop!
            let value = decode_loc_item(&item, bytes);

            for (item_key, packet_value) in subcom {
                let item_value = decode_loc_item(item_key, bytes);

                if value == item_value {
                    identify_locpacket_helper(packet_value, bytes, loc_layout);
                    break;
                }
            }
        },

        // NOTE an optimization here would be to use a hashmap, or
        // to keep only values used in decisions, determined beforehand.
        PacketDef::Array(size, packet) => {
            let mut num_elements : usize = 0;
            match size {
                ArrSize::Fixed(num) =>
                    num_elements = *num,

                ArrSize::Var(name)  => {
                    for elem in loc_layout.loc_items.iter() {
                        // NOTE matches the first item with the name
                        // might need better matching
                        if name == &elem.name[elem.name.len() - 1] {
                            let point = decode_loc_item(&elem, bytes);
                            num_elements = point.val.value() as usize;
                            break;
                        }
                    }
                }
            }

            // NOTE If Var, and name is not found, the array will be skipped
            for _ in 0..num_elements {
                identify_locpacket_helper(packet, bytes, loc_layout);
            }
        }

        PacketDef::Leaf(layer_loc_layout) => {
            // NOTE use of clone
            loc_layout.loc_items.push(layer_loc_layout.clone());
        },
    }
}

// Find all locations in a packet where a choice is made based
// on a value in the packet.
// This results in a map which initially maps each telemetry point
// name to None, but can be updated with particular values when
// decoding a particular packet.
pub fn choice_points(packet : &LayoutPacketDef) -> ChoicePoints {
    // NOTE consider passing a single hashmap around instead
    // of using 'extend' on subtree's choice points.
    let mut map = HashMap::new();

    match packet {
        PacketDef::Seq(packets) => {
            for packet in packets {
                map.extend(choice_points(packet))
            }
        },

        PacketDef::Subcom(item, subcom) => {
            map.insert(item.name.clone(), None);
            for pair in subcom {
                map.extend(choice_points(&pair.1))
            }
        },

        PacketDef::Array(size, packet) => {
            match size {
                ArrSize::Var(name)  => {
                    map.insert(name.clone(), None);
                },

                _ => (),
            }
            map.extend(choice_points(packet));
        }

        PacketDef::Leaf(_) => {
            ()
        },
    } 

    map
}

