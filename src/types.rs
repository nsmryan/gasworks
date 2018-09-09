#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::BTreeMap;

//extern crate bitreader;
//use bitreader::BitReader;

use std::fmt;

extern crate bytes;
#[allow(unused_imports)]
use self::bytes::{Bytes, Buf};


pub type Name = String;

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum Endianness {
    BigEndian,
    LittleEndian,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum IntSize {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

impl IntSize {
  pub fn num_bytes(int_size : &IntSize) -> u64 {
    match int_size {
      IntSize::Bits8  => 1,
      IntSize::Bits16 => 2,
      IntSize::Bits32 => 4,
      IntSize::Bits64 => 8,
    }
  }
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum Signedness {
    Unsigned,
    Signed,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum FloatPrim {
    F32(Endianness),
    F64(Endianness),
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct IntPrim {
    pub size : IntSize,
    pub signedness : Signedness,
    pub endianness : Endianness,
}

impl IntPrim {
  pub fn new(size : IntSize,
             signedness : Signedness,
             endianness : Endianness) -> Self {
    
    IntPrim{ size : size,
             signedness : signedness,
             endianness : endianness
           }
  }

  pub fn u8_be() -> Self {
    IntPrim::new(IntSize::Bits8, Signedness::Unsigned, Endianness::BigEndian)
  }

  pub fn u8_le() -> Self {
    IntPrim::new(IntSize::Bits8, Signedness::Unsigned, Endianness::LittleEndian)
  }

  pub fn u16_be() -> Self {
    IntPrim::new(IntSize::Bits16, Signedness::Unsigned, Endianness::BigEndian)
  }

  pub fn u16_le() -> Self {
    IntPrim::new(IntSize::Bits16, Signedness::Unsigned, Endianness::LittleEndian)
  }

  pub fn u32_be() -> Self {
    IntPrim::new(IntSize::Bits32, Signedness::Unsigned, Endianness::BigEndian)
  }

  pub fn u32_le() -> Self {
    IntPrim::new(IntSize::Bits32, Signedness::Unsigned, Endianness::LittleEndian)
  }

  pub fn u64_be() -> Self {
    IntPrim::new(IntSize::Bits64, Signedness::Unsigned, Endianness::BigEndian)
  }

  pub fn u64_le() -> Self {
    IntPrim::new(IntSize::Bits64, Signedness::Unsigned, Endianness::LittleEndian)
  }

  pub fn i8_be() -> Self {
    IntPrim::new(IntSize::Bits8, Signedness::Unsigned, Endianness::BigEndian)
  }

  pub fn i8_le() -> Self {
    IntPrim::new(IntSize::Bits8, Signedness::Signed, Endianness::LittleEndian)
  }

  pub fn i16_be() -> Self {
    IntPrim::new(IntSize::Bits16, Signedness::Signed, Endianness::BigEndian)
  }

  pub fn i16_le() -> Self {
    IntPrim::new(IntSize::Bits16, Signedness::Signed, Endianness::LittleEndian)
  }

  pub fn i32_be() -> Self {
    IntPrim::new(IntSize::Bits32, Signedness::Signed, Endianness::BigEndian)
  }

  pub fn i32_le() -> Self {
    IntPrim::new(IntSize::Bits32, Signedness::Signed, Endianness::LittleEndian)
  }

  pub fn i64_be() -> Self {
    IntPrim::new(IntSize::Bits64, Signedness::Signed, Endianness::BigEndian)
  }

  pub fn i64_le() -> Self {
    IntPrim::new(IntSize::Bits64, Signedness::Signed, Endianness::LittleEndian)
  }
}

// NOTE bits could be allow to be any size.
// currently limited to 8/16/32/64 fields
#[derive(Eq, PartialEq, Debug, Hash)]
pub struct BitPrim {
    pub entries : Vec<(Name, u32, IntPrim)>,
    pub num_bytes : IntSize,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct Enum {
    pub map : BTreeMap<i64, Name>,
    pub int_prim : IntPrim,
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum Prim {
    Int(IntPrim),
    Float(FloatPrim),
    //Bytes(usize),
    Enum(Enum),
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct Item {
    pub name : Name,
    pub typ : Prim,
}

impl Item {
  pub fn new(name : Name, typ : Prim) -> Self {
    Item{name : name, typ : typ}
  }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Layout {
    Prim(Item),
    Seq(Vec<Layout>),
    All(Vec<Layout>),
    // maybe Placement(u64, Layout)
    Bits(BitPrim),
}

#[derive(Eq, PartialEq, Debug)]
pub enum Packet {
    Seq(Vec<Packet>),
    Subcom(HashMap<Vec<Item>, Packet>),
    Layout(Layout),
}

#[derive(Eq, PartialEq, Debug)]
pub enum Protocol {
    Seq(Vec<Protocol>),
    Branch(Vec<(Vec<Prim>, Protocol)>),
    Layout(Layout),
    Packet(Packet),
}

pub type Loc = usize;

pub type LayoutMap = HashMap<Name, (Loc, Prim)>;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Value {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    //Bytes(&[u8]),
    Enum(Name, i64),
}

impl fmt::Display for Value {
  fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
    match self {
      Value::U8(value)         => write!(f, "{}", value),
      Value::U16(value)        => write!(f, "{}", value),
      Value::U32(value)        => write!(f, "{}", value),
      Value::U64(value)        => write!(f, "{}", value),
      Value::I8(value)         => write!(f, "{}", value),
      Value::I16(value)        => write!(f, "{}", value),
      Value::I32(value)        => write!(f, "{}", value),
      Value::I64(value)        => write!(f, "{}", value),
      Value::F32(value)        => write!(f, "{}", value),
      Value::F64(value)        => write!(f, "{}", value),
      Value::Enum(name, value) => write!(f, "{}", value),
    }
  }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Point {
    pub name : Name,
    pub val : Value,
}

pub type ValueMap = HashMap<Name, Value>;


impl Value {
    // NOTE this would work better with an IntValue separate
    // from the Value type
    pub fn value(&self) -> i64 {
        match self {
            Value::U8(int)  =>   *int as i64,
            Value::U16(int) =>   *int as i64,
            Value::U32(int) =>   *int as i64,
            Value::U64(int) =>   *int as i64,
            Value::I8(int)  =>   *int as i64,
            Value::I16(int) =>   *int as i64,
            Value::I32(int) =>   *int as i64,
            Value::I64(int) =>   *int as i64,
            Value::F32(int) =>   panic!("Found an F32 in a value, expecting an int!"),
            Value::F64(int) =>   panic!("Found an F64 in a value, expecting an int!"),
            //Value::Bytes(_) =>   panic!("Found an Bytes in a value, expecting an int!"),
            Value::Enum(_, _) => panic!("Found an Enum in a value, expecting an int!"),
        }
    }
}
