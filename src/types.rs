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

type LocPath = Vec<Name>;

#[derive(Eq, PartialEq, Debug, Hash, Clone, Deserialize, Serialize)]
pub struct LocItem {
  pub name : LocPath,
  pub typ : Prim,
  pub loc : Loc,
}

impl NumBytes for LocItem {
  fn num_bytes(&self) -> u64 {
    self.typ.num_bytes()
  }
}

impl LocItem {
  pub fn new(name : LocPath, typ : Prim, loc : Loc) -> LocItem {
    LocItem{ name : name, typ : typ, loc : loc }
  }
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct LocLayout {
    pub loc_items : Vec<LocItem>,
}

impl LocLayout {
    pub fn new() -> LocLayout {
        LocLayout { loc_items : Vec::new() }
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
pub enum Layout {
    Prim(Item),
    Seq(Name, Vec<Layout>),
    All(Name, Vec<Layout>),
    Array(Name, u64, Box<Layout>),
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

      Layout::Seq(_, layouts) => {
        let mut num_bytes = 0;
        // NOTE could use a fold here
        for layout in layouts.iter() {
          num_bytes += layout.num_bytes();
        }
        num_bytes
      },

      Layout::All(_, layouts) => {
        let mut num_bytes = 0;
        for layout in layouts.iter() {
          num_bytes = cmp::max(num_bytes, layout.num_bytes())
        }
        num_bytes
      },

      Layout::Array(_, size, layout) => {
          size * layout.num_bytes()
      }

      Layout::Bits(bit_prim) => {
        bit_prim.num_bytes()
      },
    }
  }
}

impl Layout {
  // NOTE the section/all/array name is not inserted here, only
  // primitives get added.
  pub fn names(&self) -> HashSet<&Name> {
    let mut names : HashSet<&Name> = HashSet::new();

    match self {
      Layout::Prim(Item{name, typ : _}) => {
        names.insert(name);
      }

      Layout::Seq(_, layouts) => {
        for layout in layouts.iter() {
          names.extend(layout.names());
        }
      },

      Layout::All(_, layouts) => {
        for layout in layouts.iter() {
          names.extend(layout.names());
        }
      },

      Layout::Array(_, _, layout) => {
          names.extend(layout.names());
      }

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
    let path = Vec::new();
    self.locate_loc(&mut loc_items, &path, &mut loc);

    LocLayout { loc_items : loc_items }
  }

  pub fn locate_loc(&self,
                    loc_items : &mut Vec<LocItem>,
                    path : &LocPath,
                    loc : &mut Loc) {
    match self {
        Layout::Prim(item) => {
            let mut item_path = path.to_vec();
            item_path.push(item.name.to_string());

            let typ = item.typ.clone();

            loc_items.push(LocItem::new(item_path, typ, *loc));
            *loc += item.typ.num_bytes();
        },

        Layout::Seq(name, layouts) => {
            let mut seq_path = path.to_vec();
            seq_path.push(name.to_string());

            for layout in layouts.iter() {
                layout.locate_loc(loc_items, &seq_path, loc);
            }
        },

        Layout::All(name, layouts) => {
            let mut all_path = path.to_vec();
            all_path.push(name.to_string());

            let mut max_loc = *loc;
            let starting_loc = *loc;

            for layout in layouts.iter() {
                *loc = starting_loc;
                layout.locate_loc(loc_items, &all_path, loc);

                // check if this layout is the largest so far
                let new_loc = layout.num_bytes();
                if new_loc > max_loc {
                    max_loc = new_loc;
                }
            }

            *loc = max_loc;
        },

        Layout::Array(name, size, layout) => {
            for index in 0 .. *size {
                let mut array_path = path.to_vec();

                // NOTE This is likely not the best way to do this
                let mut array_name = format!("{}[{}]", name, index);
                array_path.push(array_name);

                layout.locate_loc(loc_items, &array_path, loc);
            }
        }
        
        Layout::Bits(_bits) => {
          // NOTE implement Bits into LocItems
          unimplemented!();
        }
    }
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
    packet : LayoutPacketDef,
    required : HashMap<Name, Value>,
    //limits : HashMap<Name, Limit>
    //expected : HashMap<Name, Value>,
    //derived : HashMap<Name, Expr>,
}

pub type LocPacketDef = PacketDef<LocItem>;

pub type LayoutPacketDef = PacketDef<Item>;

impl NumBytes for LayoutPacketDef {
    fn num_bytes(&self) -> u64 {
        let mut num_bytes : u64 = 0;

        match self {
            PacketDef::Seq(_, packets) => {
                for packet in packets {
                    num_bytes += packet.num_bytes();
                }
            },

            PacketDef::Subcom(_, _, pairs) => {
                let mut subcom_bytes :u64 = 0;
                for (_, packet) in pairs {
                    cmp::max(subcom_bytes, packet.num_bytes());
                }
                num_bytes += subcom_bytes;
            },

            PacketDef::Array(_, size, packet) => {
                match size {
                    ArrSize::Var(_) => {
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
        let mut names : HashSet<&Name> = HashSet::new();
        match self {
            PacketDef::Seq(_, packets) => {
                for packet in packets {
                    names.extend(packet.names());
                }
            },

            PacketDef::Subcom(_, _, pairs) => {
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
        let mut loc_layout : LocLayout = LocLayout::new();
        let mut loc_path = LocPath::new();

        let result = LayoutPacketDef::locate_helper(self,
                                                    &mut offset,
                                                    &mut loc_layout,
                                                    &mut loc_path); 

        match result {
            Some(loc_layout) => Some(*loc_layout),
            None => None,
        }
    }
    
    fn locate_helper<'a>(packet : &LayoutPacketDef, 
                     offset : &mut u64, 
                     loc_layout : &'a mut LocLayout,
                     loc_path : &mut LocPath) -> Option<&'a mut LocLayout> {
        let mut result : Option<&'a mut LocLayout> = None;

        match packet {
            PacketDef::Seq(name, packets) => {
                let mut locate_result;
                for packet in packets {
                    loc_path.push(name.to_string());
                    locate_result = LayoutPacketDef::locate_helper(packet,
                                                                   offset,
                                                                   loc_layout,
                                                                   loc_path);
                    loc_path.pop();

                    match locate_result {
                        Some(loc_layout) => result = Some(loc_layout),
                        None => {
                            result = None;
                            break;
                        },
                    }
                }
            },

            PacketDef::Subcom(_, _, _) => {
                result = None;
            },

            PacketDef::Array(_, _, _) => {
                result = None;
            }

            PacketDef::Leaf(ref item) => {
                loc_path.push(item.name.clone());
                loc_layout.loc_items.push(LocItem::new(loc_path.clone(),
                                                       item.typ.clone(),
                                                       *offset));
                loc_path.pop();
                *offset += item.num_bytes();

                result = Some(loc_layout);
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

#[derive(PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
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
      Value::F32(value)        => write!(f, "{:.3}", value),
      Value::F64(value)        => write!(f, "{:.3}", value),
      Value::Enum(_, value) => write!(f, "{}", value),
    }
  }
}

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

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct ValueMap {
    pub value_map : FnvHashMap<Name, ValueEntry>,
}

impl ValueMap {
    pub fn new(value_map : FnvHashMap<Name, ValueEntry>) -> ValueMap {
        ValueMap { value_map : value_map }
    }

    pub fn values(&self) -> Vec<&Value> {
        let mut values = Vec::new();

        for value_entry in self.value_map.values() {
            match value_entry {
                ValueEntry::Leaf(value) => {
                    values.push(value);
                }

                ValueEntry::Section(value_map) => {
                    values.extend(value_map.values());
                }

                ValueEntry::Array(array) => {
                    for value_map in array {
                        values.extend(value_map.values());
                    }
                }
            }
        }

        values
    }

    // NOTE this returns the Value, not a reference. is this okay?
    pub fn lookup(&self, name : &Name) -> Option<Value> {
        match self.value_map.get(name) {
            Some(ValueEntry::Leaf(value)) =>
                return Some(value.clone()),

            None => {
                for value_entry in self.value_map.values() {
                    match value_entry {
                        ValueEntry::Leaf(_) => (),

                        ValueEntry::Section(value_map) => {
                            match value_map.lookup(&name.clone()) {
                                Some(value) => {
                                    return Some(value);
                                }
                                None => ()
                            }
                        },

                        // if the name is in an array, it is not clear which one to choose,
                        // so don't even look.
                        ValueEntry::Array(_) => {
                            ()
                        }
                    }
                }
            }

            _ => (),
        }

        return None;
    }
}

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub enum ValueEntry {
    Leaf(Value),
    Section(ValueMap),
    Array(Vec<ValueMap>),
}

