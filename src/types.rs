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

use std::fmt;
use std::cmp;

use self::fnv::FnvHashMap;

#[allow(unused_imports)]
use self::bytes::{Bytes, Buf};

use prim::*;
use layout::*;
use value::*;


pub trait NumBytes {
  fn num_bytes(&self) -> u64;
}

pub type Name = String;

pub type Loc = u64;

pub type ChoicePoints = HashMap<Name, Option<Value>>;


#[derive(Eq, PartialEq, Debug, Hash, Deserialize, Serialize)]
pub struct Item {
    pub name: Name,
    pub typ: Prim,
}

impl Clone for Item {
    fn clone(&self) -> Item {
        Item {name: self.name.clone(),
             typ: self.typ.clone(),
        }
    }
}

impl NumBytes for Item {
  fn num_bytes(&self) -> u64 {
    self.typ.num_bytes()
  }
}

impl Item {
  pub fn new(name: Name, typ: Prim) -> Self {
    Item{name: name, typ: typ}
  }
}

pub type LocPath = Vec<Name>;

#[derive(Eq, PartialEq, Debug, Hash, Clone, Deserialize, Serialize)]
pub struct LocItem {
  pub name: LocPath,
  pub typ: Prim,
  pub loc: Loc,
}

impl NumBytes for LocItem {
  fn num_bytes(&self) -> u64 {
    self.typ.num_bytes()
  }
}

impl LocItem {
  pub fn new(name: LocPath, typ: Prim, loc: Loc) -> LocItem {
    LocItem{ name: name, typ: typ, loc: loc }
  }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct LocLayout {
    pub loc_items: Vec<LocItem>,
}

impl LocLayout {
    pub fn new() -> LocLayout {
        LocLayout { loc_items: Vec::new() }
    }
}

impl NumBytes for LocLayout {
    fn num_bytes(&self) -> u64 {
        let mut num_bytes = 0;

        for loc_item in &self.loc_items {
            num_bytes = cmp::max(num_bytes, loc_item.loc + loc_item.typ.num_bytes());
        }

        num_bytes
    }
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub enum ArrSize {
    Fixed(usize),
    Var(Name),
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
    required: HashMap<Name, Value>,
    //limits: HashMap<Name, Limit>
    //expected: HashMap<Name, Value>,
    //derived: HashMap<Name, Expr>,
}

pub type LocPacketDef = PacketDef<LocItem>;

pub type LayoutPacketDef = PacketDef<Item>;

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

#[derive(Eq, PartialEq, Debug)]
pub enum Protocol<T> {
    Seq(Vec<Protocol<T>>),
    // NOTE extend to multiple item/value pairs.
    // current restriction to single item is for simplicity
    Branch(LocItem, Vec<(LocItem, Protocol<T>)>),
    // NOTE maybe could become LocItem and only decode necessary items
    Layout(Layout),
    Leaf(T),
}

pub type LayoutMap = FnvHashMap<Name, (Loc, Prim)>;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Point {
    pub name: Name,
    pub val: Value,
}

impl Point {
    pub fn new(name: Name, val: Value) -> Point {
        Point { name: name, val: val }
    }
}

#[derive(Debug)]
pub struct PacketStream<'a> {
    loc_layout: LocLayout,
    bytes: &'a Vec<u8>,
    position: usize,
    num_bytes: usize,
}

impl<'a> PacketStream<'a> {
    pub fn new(packet: LayoutPacketDef, bytes: &'a Vec<u8>) -> PacketStream {
        let loc_layout = packet.locate().unwrap();
        PacketStream { loc_layout: loc_layout,
                       bytes: bytes,
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

