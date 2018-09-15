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

extern crate bytes;
#[allow(unused_imports)]
use self::bytes::{Bytes, Buf};

pub trait NumBytes {
  fn num_bytes(&self) -> u64;
}

pub type Name = String;

pub type Loc = u64;

pub type ChoicePoints = HashMap<Name, Option<Value>>;

#[derive(Eq, PartialEq, Debug, Hash, Clone, Deserialize, Serialize)]
pub enum Endianness {
    BigEndian,
    LittleEndian,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Deserialize, Serialize)]
pub enum IntSize {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

impl NumBytes for IntSize {
  fn num_bytes(&self) -> u64 {
    match self {
      IntSize::Bits8  => 1,
      IntSize::Bits16 => 2,
      IntSize::Bits32 => 4,
      IntSize::Bits64 => 8,
    }
  }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Deserialize, Serialize)]
pub enum Signedness {
    Unsigned,
    Signed,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Deserialize, Serialize)]
pub enum FloatPrim {
    F32(Endianness),
    F64(Endianness),
}

impl NumBytes for FloatPrim {
  fn num_bytes(&self) -> u64 {
    match self {
      FloatPrim::F32(_) => 4,
      FloatPrim::F64(_) => 8,
    }
  }
}

impl FloatPrim {
    pub fn f32_be() -> FloatPrim {
        FloatPrim::F32(Endianness::BigEndian)
    }
    pub fn f32_le() -> FloatPrim {
        FloatPrim::F32(Endianness::LittleEndian)
    }
    pub fn f64_be() -> FloatPrim {
        FloatPrim::F64(Endianness::BigEndian)
    }
    pub fn f64_le() -> FloatPrim {
        FloatPrim::F64(Endianness::LittleEndian)
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Deserialize, Serialize)]
pub struct IntPrim {
    pub size : IntSize,
    pub signedness : Signedness,
    pub endianness : Endianness,
}

impl NumBytes for IntPrim {
  fn num_bytes(&self) -> u64 {
    self.size.num_bytes()
  }
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
#[derive(Eq, PartialEq, Debug, Hash, Clone, Deserialize, Serialize)]
pub struct BitPrim {
    pub entries : Vec<(Name, u32, IntPrim)>,
    // NOTE rename to size or int_prim
    pub num_bytes : u64,
}

impl NumBytes for BitPrim {
  fn num_bytes(&self) -> u64 {
    self.num_bytes
  }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Deserialize, Serialize)]
pub struct Enum {
    pub map : BTreeMap<i64, Name>,
    pub int_prim : IntPrim,
}

impl NumBytes for Enum {
  fn num_bytes(&self) -> u64 {
    self.int_prim.num_bytes()
  }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Deserialize, Serialize)]
pub enum Prim {
    Int(IntPrim),
    Float(FloatPrim),
    //Bytes(usize),
    Enum(Enum),
}

impl NumBytes for Prim {
  fn num_bytes(&self) -> u64 {
    match self {
      Prim::Int(int_prim)     => int_prim.num_bytes(),
      Prim::Float(float_prim) => float_prim.num_bytes(),
      Prim::Enum(enum_prim)   => enum_prim.num_bytes(),
    }
  }
}

#[derive(Eq, PartialEq, Debug, Hash, Deserialize, Serialize)]
pub struct Item {
    pub name : Name,
    pub typ : Prim,
}

impl Clone for Item {
    fn clone(&self) -> Item {
        Item {name : self.name.clone(),
             typ  : self.typ.clone(),
        }
    }
}

impl NumBytes for Item {
  fn num_bytes(&self) -> u64 {
    self.typ.num_bytes()
  }
}

impl Item {
  pub fn new(name : Name, typ : Prim) -> Self {
    Item{name : name, typ : typ}
  }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Deserialize, Serialize)]
pub struct LocItem {
  pub name : Name,
  pub typ : Prim,
  pub loc : Loc,
}

impl NumBytes for LocItem {
  fn num_bytes(&self) -> u64 {
    self.typ.num_bytes()
  }
}

impl LocItem {
  pub fn new(name : Name, typ : Prim, loc : Loc) -> LocItem {
    LocItem{ name : name, typ : typ, loc : loc }
  }
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct LocLayout {
    pub loc_items : Vec<LocItem>,
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
pub enum Layout {
    Prim(Item),
    Seq(Vec<Layout>),
    All(Vec<Layout>),
    // NOTE consider whether Placements still make sense.
    // they can be encoded by buffers and Alls
    // Placement(u64, Layout)
    Bits(BitPrim),
}

impl NumBytes for Layout {
  fn num_bytes(&self) -> u64 {
    match self {
      Layout::Prim(item) => {
        item.num_bytes()
      }

      Layout::Seq(layouts) => {
        let mut num_bytes = 0;
        // NOTE could use a fold here
        for layout in layouts.iter() {
          num_bytes += layout.num_bytes();
        }
        num_bytes
      },

      Layout::All(layouts) => {
        let mut num_bytes = 0;
        for layout in layouts.iter() {
          num_bytes = cmp::max(num_bytes, layout.num_bytes())
        }
        num_bytes
      },

      Layout::Bits(bit_prim) => {
        bit_prim.num_bytes()
      },
    }
  }
}

impl Layout {
  pub fn names(&self) -> HashSet<&Name> {
    let mut names : HashSet<&Name> = HashSet::new();

    match self {
      Layout::Prim(Item{name : name, typ : _}) => {
        names.insert(name);
      }

      Layout::Seq(layouts) => {
        for layout in layouts.iter() {
          names.extend(layout.names());
        }
      },

      Layout::All(layouts) => {
        for layout in layouts.iter() {
          names.extend(layout.names());
        }
      },

      Layout::Bits(bit_prims) => {
        for bit_prim in bit_prims.entries.iter() {
          names.insert(&bit_prim.0);
        }
      },
    }

    names
  }

  pub fn locate(&self) -> LocLayout {
    let mut loc = 0;
    let mut loc_items = Vec::new();
    self.locate_loc(&mut loc_items, &mut loc);

    LocLayout { loc_items : loc_items }
  }

  pub fn locate_loc(&self, loc_items : &mut Vec<LocItem>, loc : &mut Loc) {
    match self {
        Layout::Prim(item) => {
            let typ = item.typ.clone();
            loc_items.push(LocItem::new(item.name.to_string(), typ, *loc));
            *loc += item.typ.num_bytes();
        },

        Layout::Seq(layouts) => {
            for layout in layouts.iter() {
                layout.locate_loc(loc_items, loc);
            }
        },

        Layout::All(layouts) => {
            let mut max_loc = *loc;
            let starting_loc = *loc;

            for layout in layouts.iter() {
                *loc = starting_loc;
                layout.locate_loc(loc_items, loc);

                // check if this layout is the largest so far
                let new_loc = layout.num_bytes();
                if new_loc > max_loc {
                    max_loc = new_loc;
                }
            }

            *loc = max_loc;
        },
        
        Layout::Bits(_bits) => {
          // NOTE implement Bits into LocItems
          unimplemented!();
        }
    }
  }
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub enum Packet<T> {
    Seq(Vec<Packet<T>>),
    // NOTE add back in multiple items here when needed. removed for simplicity.
    // Subcom(HashMap<Vec<Item>, Packet>),
    Subcom(T, Vec<(T, Packet<T>)>),
    Leaf(T),
}

pub type LocPacket = Packet<LocItem>;

pub type LayoutPacket = Packet<Item>;

impl LayoutPacket {
    // NOTE this function does not work! it does not create the 
    // correct locations for LocItems!
    pub fn locate(&self) -> LocPacket {
        match self {
            Packet::Seq(packets) => {
                Packet::Seq(packets.iter().map(|packet| {packet.locate()}).collect())
            },

            Packet::Subcom(item, pairs) => {
                // NOTE use of clone
                Packet::Subcom(LocItem::new(item.name.clone(), item.typ.clone(), 0),
                               pairs.iter().map(|(item, packet)| {
                                   (LocItem::new(item.name.clone(), item.typ.clone(), 0),
                                    packet.locate())
                               }).collect()
                )
            },

            Packet::Leaf(ref item) => {
                // NOTE use of clone
                let prim = Layout::Prim(item.clone());
                let seq = Layout::Seq(vec!(prim));
                // NOTE use of clone
                Packet::Leaf(seq.locate().loc_items[0].clone())
            },
        }
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

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
          Value::U8(value)         => Value::U8(*value),
          Value::U16(value)        => Value::U16(*value),
          Value::U32(value)        => Value::U32(*value),
          Value::U64(value)        => Value::U64(*value),
          Value::I8(value)         => Value::I8(*value),
          Value::I16(value)        => Value::I16(*value),
          Value::I32(value)        => Value::I32(*value),
          Value::I64(value)        => Value::I64(*value),
          Value::F32(value)        => Value::F32(*value),
          Value::F64(value)        => Value::F64(*value),
          Value::Enum(name, value) => Value::Enum(name.clone(), *value),
        }
    }
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
      Value::Enum(_, value) => write!(f, "{}", value),
    }
  }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Point {
    pub name : Name,
    pub val : Value,
}

impl Point {
    pub fn new(name : Name, val : Value) -> Point {
        Point { name : name, val : val }
    }
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
            Value::F32(_) =>   panic!("Found an F32 in a value, expecting an int!"),
            Value::F64(_) =>   panic!("Found an F64 in a value, expecting an int!"),
            //Value::Bytes(_) =>   panic!("Found an Bytes in a value, expecting an int!"),
            Value::Enum(_, _) => panic!("Found an Enum in a value, expecting an int!"),
        }
    }
}
