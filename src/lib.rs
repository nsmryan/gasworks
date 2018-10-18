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
        PacketDef::Seq(name, packets) => {
            for packet in packets {
                identify_locpacket_helper(packet, bytes, loc_layout);
            }
        },

        PacketDef::Subcom(name, item, subcom) => {
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
        PacketDef::Array(name, size, packet) => {
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
pub fn choice_points(packet: &LayoutPacketDef) -> ChoicePoints {
    let mut map = HashMap::new();

    choice_points_helper(packet, &mut map);

    map
}

pub fn choice_points_helper(packet: &LayoutPacketDef, map: &mut ChoicePoints) {
    match packet {
        PacketDef::Seq(name, packets) => {
            for packet in packets {
                choice_points_helper(packet, map);
            }
        },

        PacketDef::Subcom(name, item, subcom) => {
            map.insert(item.name.clone(), None);
            for pair in subcom {
                choice_points_helper(&pair.1, map);
            }
        },

        PacketDef::Array(name, size, packet) => {
            match size {
                ArrSize::Var(name)  => {
                    map.insert(name.clone(), None);
                },

                _ => (),
            }
            choice_points_helper(packet, map);
        }

        PacketDef::Leaf(_) => {
            ()
        },
    } 
}

