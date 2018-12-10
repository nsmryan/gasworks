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

use types::{NumBytes, Name};


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
    pub size: IntSize,
    pub signedness: Signedness,
    pub endianness: Endianness,
}

impl NumBytes for IntPrim {
  fn num_bytes(&self) -> u64 {
    self.size.num_bytes()
  }
}

impl IntPrim {
  pub fn new(size: IntSize,
             signedness: Signedness,
             endianness: Endianness) -> Self {
    
    IntPrim{ size: size,
             signedness: signedness,
             endianness: endianness
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
    pub entries: Vec<(Name, u32, IntPrim)>,
    // NOTE rename to size or int_prim
    pub num_bytes: u64,
}

impl NumBytes for BitPrim {
  fn num_bytes(&self) -> u64 {
    self.num_bytes
  }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Deserialize, Serialize)]
pub struct Enum {
    pub map: BTreeMap<i64, Name>,
    pub int_prim: IntPrim,
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

