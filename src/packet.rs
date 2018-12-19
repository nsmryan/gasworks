extern crate bytes;
extern crate fnv;

#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::BTreeMap;
#[allow(unused_imports)]
use std::option;

use std::cmp;


#[allow(unused_imports)]
use self::bytes::{Bytes, Buf};

use prim::*;
use types::*;
use loclayout::*;


#[derive(Debug)]
pub struct PacketStream<'a> {
    bytes: &'a Vec<u8>,
    position: usize,
    num_bytes: usize,
}

impl<'a> PacketStream<'a> {
    pub fn new(packet: LayoutPacketDef, bytes: &'a Vec<u8>) -> PacketStream {
        PacketStream { bytes: bytes,
                       position: 0,
                       num_bytes: packet.num_bytes() as usize,
        }
    }
}

impl<'a> Iterator for PacketStream<'a> {
    type Item = &'a[u8];

    fn next(&mut self) -> Option<&'a[u8]> {
        if (self.position + self.num_bytes) < self.bytes.len() {
            let prev_position = self.position;

            self.position += self.num_bytes;

            Some(&self.bytes[prev_position..(prev_position + self.num_bytes)])
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub enum PacketDef<T> {
    Seq(Name, Vec<PacketDef<T>>),
    // NOTE add back in multiple items here when needed. removed for simplicity.
    // Subcom(HashMap<Vec<Item>, PacketDef>),
    Subcom(Name, T, Vec<(T, PacketDef<T>)>),
    Array(Name, ArrSize, Box<PacketDef<T>>),
    Leaf(T),
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Packet {
    packet: LayoutPacketDef,
    //required: HashMap<Name, Value>,
    //limits: HashMap<Name, Limit>
    //expected: HashMap<Name, Value>,
    //derived: HashMap<Name, Expr>,
}

pub type LayoutPacketDef = PacketDef<Item>;

pub type LocPacketDef = PacketDef<LocItem>;

impl NumBytes for LayoutPacketDef {
    fn num_bytes(&self) -> u64 {
        let mut num_bytes: u64 = 0;

        match self {
            PacketDef::Seq(name, packets) => {
                for packet in packets {
                    num_bytes += packet.num_bytes();
                }
            },

            PacketDef::Subcom(name, item, pairs) => {
                let mut subcom_bytes:u64 = 0;
                for (_, packet) in pairs {
                    cmp::max(subcom_bytes, packet.num_bytes());
                }
                num_bytes += subcom_bytes;
            },

            PacketDef::Array(name, size, packet) => {
                match size {
                    ArrSize::Var(name) => {
                        panic!("can't statically determine num_bytes for variable size array!");
                    },

                    ArrSize::Fixed(num_elements) => {
                        num_bytes += packet.num_bytes() * (*num_elements as u64);
                    }
                }
            },

            PacketDef::Leaf(item) => {
               num_bytes += item.num_bytes();
            },
        }

        num_bytes
    }
}

impl LayoutPacketDef {
    pub fn names(&self) -> HashSet<&Name> {
        let mut names: HashSet<&Name> = HashSet::new();
        match self {
            PacketDef::Seq(name, packets) => {
                for packet in packets {
                    names.extend(packet.names());
                }
            },

            PacketDef::Subcom(_, item, pairs) => {
                for (_, packet) in pairs {
                   names.extend(packet.names());
                }
            },

            PacketDef::Array(_, _, packet) => {
               names.extend(packet.names());
            },

            PacketDef::Leaf(item) => {
               names.insert(&item.name);
            },
        }

        names
    }

    // NOTE this function does not work! it does not create the 
    // correct locations for LocItems!
    pub fn locate(&self) -> Option<LocLayout> {
        let mut offset = 0;
        let mut loc_layout: LocLayout = LocLayout::new();
        let mut loc_path = LocPath::new();

        let result = LayoutPacketDef::locate_helper(self,
                                                    &mut offset,
                                                    &mut loc_layout,
                                                    &mut loc_path); 

        if result {
            Some(loc_layout)
        }
        else {
            None
        }
    }
    
    fn locate_helper(packet:      &LayoutPacketDef, 
                     offset:      &mut u64, 
                     loc_layout:  &mut LocLayout,
                     loc_path:    &mut LocPath) -> bool {
        let mut result: bool;

        match packet {
            PacketDef::Seq(name, packets) => {
                result = true;

                for packet in packets {
                    loc_path.push(name.to_string());
                    result = LayoutPacketDef::locate_helper(packet,
                                                            offset,
                                                            loc_layout,
                                                            loc_path);
                    loc_path.pop();
                    if !result {
                        break;
                    }
                }
            },

            PacketDef::Subcom(_, _, _) => {
                result = false;
            },

            PacketDef::Array(_, _, _) => {
                result = false;
            }

            PacketDef::Leaf(ref item) => {
                loc_path.push(item.name.clone());
                loc_layout.loc_items.push(LocItem::new(loc_path.clone(),
                                                       item.typ.clone(),
                                                       *offset));
                loc_path.pop();
                *offset += item.num_bytes();

                result = true;
            },
        }

        result
    }
}
