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

use self::fnv::FnvHashMap;

#[allow(unused_imports)]
use self::bytes::{Bytes, Buf};

use prim::*;
use layout::*;
use value::*;
use packet::*;


pub trait NumBytes {
  fn num_bytes(&self) -> u64;
}

pub type Name = String;

pub type Loc = u64;

pub type ChoicePoints = HashMap<Name, Option<Value>>;

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

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub enum ArrSize {
    Fixed(usize),
    Var(Name),
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


// Definitions for limit monitoring
type Persistence = usize;
type Time = f64;

enum LimitState {
    Okay,
    Exceeded(Time, Persistence),
    Triggered(Time, Persistence),
}

enum Comparison {
    LT,
    GT,
    EQ,
    WITHIN,
    WITHOUT,
}

struct LimitDef {
    name: Name,
    persistence: Persistence,
    comparison: Comparison,
}

struct LimitInfo {
    state: LimitState,
    definition: LimitDef,
}

