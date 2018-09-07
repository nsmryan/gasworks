use std::collections::HashSet;
use std::collections::HashMap;

extern crate bytes;
use self::bytes::{Bytes, Buf};

pub type Name = String;

pub enum Endianness {
    BigEndian,
    LittleEndian,
}

pub enum IntSize {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

pub enum Signedness {
    Unsigned,
    Signed,
}

pub enum FloatPrim {
    F32(Endianness),
    F64(Endianness),
}

pub struct IntPrim {
    pub size : IntSize,
    pub signedness : Signedness,
    pub endianness : Endianness,
}

pub struct BitPrim {
    pub entries : Vec<(Name, IntPrim)>,
    pub bytes : IntSize,
}

pub struct Enum {
    pub map : HashMap<i64, Name>,
    pub int_prim : IntPrim,
}

pub enum Prim {
    Int(IntPrim),
    Float(FloatPrim),
    //Bytes(usize),
    Bits(BitPrim),
    Enum(Enum),
}

pub struct Item {
    pub name : Name,
    pub typ : Prim,
}

pub enum Layout {
    Prim(Prim),
    Seq(Vec<Layout>),
    All(Vec<Layout>),
    // maybe Placement(u64, Layout)
}

pub enum Packet {
    Seq(Vec<Packet>),
    Subcom(HashMap<HashSet<Item>, Packet>), // maybe just HashMap<Item, Packet>
    Layout(Layout),
}

pub enum Protocol {
    Seq(Vec<Protocol>),
    Branch(Vec<(HashSet<Prim>, Protocol)>),
    Layout(Layout),
    Packet(Packet),
}

pub type Loc = usize;

pub type LayoutMap = HashMap<Name, (Loc, Prim)>;

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
