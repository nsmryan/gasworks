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

use self::fnv::FnvHashMap;

#[allow(unused_imports)]
use self::bytes::{Bytes, Buf};

// use prim::*;
// use layout::*;
use types::*;


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
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct ValueMap {
    pub value_map: FnvHashMap<Name, ValueEntry>,
}

impl ValueMap {
    pub fn new(value_map: FnvHashMap<Name, ValueEntry>) -> ValueMap {
        ValueMap { value_map: value_map }
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

    pub fn lookup(&self, name: &Name) -> Option<Value> {
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

