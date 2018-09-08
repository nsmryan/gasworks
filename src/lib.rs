
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeMap;

extern crate bitreader;
use bitreader::BitReader;

extern crate bytes;
use bytes::{Bytes, Buf};

extern crate byteorder;
use byteorder::{LittleEndian, BigEndian, ByteOrder};

use std::io::{Cursor, Read};

pub mod types;
use types::*;


pub fn decode(layout : Layout, bytes : &mut Cursor<&[u8]>) -> ValueMap {
    let mut map = HashMap::new();

    let _ = decode_layout(&layout, bytes, &mut map);

    map
}

pub fn decode_prim(prim : &Prim, bytes : &mut Cursor<&[u8]>) -> Value {
    match prim {
        Prim::Int(int_prim) => {
            decode_int(int_prim, bytes)
        },

        Prim::Float(float_type) => {
            match float_type {
                FloatPrim::F32(endianness) => {
                    match endianness {
                        Endianness::BigEndian    => Value::F32(bytes.get_f32_be()),
                        Endianness::LittleEndian => Value::F32(bytes.get_f32_le()),
                    }
                }

                FloatPrim::F64(endianness) => {
                    match endianness {
                        Endianness::BigEndian    => Value::F64(bytes.get_f64_be()),
                        Endianness::LittleEndian => Value::F64(bytes.get_f64_le()),
                    }
                },
            }
        },

        // NOTE add bytes back in when you understand how to avoid copying
        //Prim::Bytes(num_bytes) => {
        //    let buf = vec![0; num_bytes];
        //    
        //    bytes.read_exact(buf.as_mut_slice()).unwrap();

        //    Value::Bytes(buf.as_slice())
        //},

        Prim::Enum(Enum{map, int_prim}) => {
            // NOTE this doesn't ensure that the int_prim
            // decodes to a value in the map, and doesn't
            // enforce that the int_value.value doesn't loose precision
            let int_value = decode_int(int_prim, bytes);
            let int = int_value.value();
            // NOTE the use of to_string here may be wrong?
            Value::Enum(map.get(&int).unwrap().to_string(), int)
        },
    }
}

pub fn decode_int(int_prim : &IntPrim, bytes : &mut Cursor<&[u8]>) -> Value {
    let IntPrim{size, signedness, endianness} = int_prim;

    match endianness {
        Endianness::BigEndian => {
            match signedness {
                Signedness::Unsigned => {
                    match size {
                        IntSize::Bits8  => Value::U8(bytes.get_u8()),
                        IntSize::Bits16 => Value::U16(bytes.get_u16_be()),
                        IntSize::Bits32 => Value::U32(bytes.get_u32_be()),
                        IntSize::Bits64 => Value::U64(bytes.get_u64_be()),
                    }
                },

                Signedness::Signed => {
                    match size {
                        IntSize::Bits8  => Value::I8(bytes.get_i8()),
                        IntSize::Bits16 => Value::I16(bytes.get_i16_be()),
                        IntSize::Bits32 => Value::I32(bytes.get_i32_be()),
                        IntSize::Bits64 => Value::I64(bytes.get_i64_be()),
                    }
                },
            }
        }

        Endianness::LittleEndian => {
            match signedness {
                Signedness::Unsigned => {
                    match size {
                        IntSize::Bits8  => Value::U8(bytes.get_u8()),
                        IntSize::Bits16 => Value::U16(bytes.get_u16_le()),
                        IntSize::Bits32 => Value::U32(bytes.get_u32_le()),
                        IntSize::Bits64 => Value::U64(bytes.get_u64_le()),
                    }
                },

                Signedness::Signed => {
                    match size {
                        IntSize::Bits8  => Value::I8(bytes.get_i8()),
                        IntSize::Bits16 => Value::I16(bytes.get_i16_le()),
                        IntSize::Bits32 => Value::I32(bytes.get_i32_le()),
                        IntSize::Bits64 => Value::I64(bytes.get_i64_le()),
                    }
                },
            }
        }
    }
}

fn decode_layout(layout : &Layout, bytes : &mut Cursor<&[u8]>, map : &mut ValueMap) {
    
    match layout {
        Layout::Prim(item) => {
            let value = decode_prim(&item.typ, bytes);
            map.insert(item.name.to_string(), value);
        },

        Layout::Seq(layouts) => {
            for layout in layouts.iter() {
                decode_layout(layout, bytes, map);
            }
        },

        Layout::All(layouts) => {
            let mut max_loc = bytes.position();
            let starting_loc = bytes.position();

            for layout in layouts.iter() {
                // jump back to the start and decode next layout
                bytes.set_position(starting_loc);
                decode_layout(layout, bytes, map);

                // check if this layout is the largest so far
                let new_loc = bytes.position();
                if new_loc > max_loc {
                    max_loc = new_loc;
                }
            }

            // jump forward past the largest layout
            bytes.set_position(max_loc);
        },
        
        Layout::Bits(BitPrim{entries, num_bytes}) => {
            let slice = bytes.get_mut();
            let mut reader = BitReader::new(slice);
            for (name, num_bits, int_prim) in entries.iter() {
                match int_prim.signedness {
                    Signedness::Unsigned => {
                        match int_prim.size {
                          IntSize::Bits8 => {
                              let int_value = Value::U8(reader.read_u8(*num_bits as u8).unwrap());
                              map.insert(name.to_string(), int_value);
                          },
                          IntSize::Bits16 => {
                              let int_value = Value::U16(reader.read_u16(*num_bits as u8).unwrap());
                              map.insert(name.to_string(), int_value);
                          },
                          IntSize::Bits32 => {
                              let int_value = Value::U32(reader.read_u32(*num_bits as u8).unwrap());
                              map.insert(name.to_string(), int_value);
                          },
                          IntSize::Bits64 => {
                              let int_value = Value::U64(reader.read_u64(*num_bits as u8).unwrap());
                              map.insert(name.to_string(), int_value);
                          },
                        }
                    },

                    Signedness::Signed => {
                        match int_prim.size {
                          IntSize::Bits8 => {
                              let int_value = Value::I8(reader.read_i8(*num_bits as u8).unwrap());
                              map.insert(name.to_string(), int_value);
                          },
                          IntSize::Bits16 => {
                              let int_value = Value::I16(reader.read_i16(*num_bits as u8).unwrap());
                              map.insert(name.to_string(), int_value);
                          },
                          IntSize::Bits32 => {
                              let int_value = Value::I32(reader.read_i32(*num_bits as u8).unwrap());
                              map.insert(name.to_string(), int_value);
                          },
                          IntSize::Bits64 => {
                              let int_value = Value::I64(reader.read_i64(*num_bits as u8).unwrap());
                              map.insert(name.to_string(), int_value);
                          },
                        }
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_prim() {
      let byte_prim = Prim::Int(IntPrim::new(IntSize::Bits8, Signedness::Unsigned, Endianness::BigEndian));

      let float32_be = Prim::Float(FloatPrim::F32(Endianness::BigEndian));
      let float64_be = Prim::Float(FloatPrim::F64(Endianness::BigEndian));

      let float32_le = Prim::Float(FloatPrim::F32(Endianness::LittleEndian));
      let float64_le = Prim::Float(FloatPrim::F64(Endianness::LittleEndian));

      //let bits_prim = Prim::Bits();

      let mut enum_map = BTreeMap::new();
      enum_map.insert(0, "Zero".to_string());
      enum_map.insert(1, "One".to_string());
      enum_map.insert(2, "Two".to_string());
      enum_map.insert(5, "Five".to_string());
      let enum_prim = IntPrim::new(IntSize::Bits32, Signedness::Unsigned, Endianness::BigEndian);
      let enum_prim = Prim::Enum(Enum{map : enum_map, int_prim : enum_prim});

      let v = vec![0xAA,
                  0x3D, 0xCC, 0xCC, 0xCD,
                  0x3F, 0xC9, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9A,
                  0xCD, 0xCC, 0xCC, 0x3D,
                  0x9A, 0x99, 0x99, 0x99, 0x99, 0x99, 0xC9, 0x3F,
                  0x00, 0x00, 0x00, 0x00,
                  0x00, 0x00, 0x00, 0x01,
                  0x00, 0x00, 0x00, 0x02,
                  0x00, 0x00, 0x00, 0x05
                  ];
      let mut bytes = Cursor::new(v.as_slice());
      let byte_value = decode_prim(&byte_prim, &mut bytes);

      let float32_value_be = decode_prim(&float32_be, &mut bytes);
      let float64_value_be = decode_prim(&float64_be, &mut bytes);

      let float32_value_le = decode_prim(&float32_le, &mut bytes);
      let float64_value_le = decode_prim(&float64_le, &mut bytes);

      let enum_value_zero = decode_prim(&enum_prim, &mut bytes);
      let enum_value_one = decode_prim(&enum_prim, &mut bytes);
      let enum_value_two = decode_prim(&enum_prim, &mut bytes);
      let enum_value_five = decode_prim(&enum_prim, &mut bytes);

      assert!(byte_value == Value::U8(0xAA));

      assert!(float32_value_be == Value::F32(0.1));
      assert!(float64_value_be == Value::F64(0.2));

      assert!(float32_value_le == Value::F32(0.1));
      assert!(float64_value_le == Value::F64(0.2));

      assert!(enum_value_zero == Value::Enum("Zero".to_string(), 0));
      assert!(enum_value_one  == Value::Enum("One".to_string(),  1));
      assert!(enum_value_two  == Value::Enum("Two".to_string(),  2));
      assert!(enum_value_five == Value::Enum("Five".to_string(), 5));
    }

    #[test]
    fn test_decode_int() {
      let byte_prim = IntPrim::new(IntSize::Bits8, Signedness::Unsigned, Endianness::BigEndian);

      let short_prim_be = IntPrim::new(IntSize::Bits16, Signedness::Unsigned, Endianness::BigEndian);
      let short_prim_le = IntPrim::new(IntSize::Bits16, Signedness::Unsigned, Endianness::LittleEndian);

      let int_prim_be = IntPrim::new(IntSize::Bits32, Signedness::Unsigned, Endianness::BigEndian);
      let int_prim_le = IntPrim::new(IntSize::Bits32, Signedness::Unsigned, Endianness::LittleEndian);

      let long_prim_be = IntPrim::new(IntSize::Bits64, Signedness::Unsigned, Endianness::BigEndian);
      let long_prim_le = IntPrim::new(IntSize::Bits64, Signedness::Unsigned, Endianness::LittleEndian);

      let v = vec![0xAA,
                   0x11, 0x22,
                   0x33, 0x44,
                   0x11, 0x22, 0x33, 0x44,
                   0x44, 0x33, 0x22, 0x11,
                   0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                   0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11
                  ];
      let mut bytes = Cursor::new(v.as_slice());
      let byte = decode_int(&byte_prim, &mut bytes);
      let short_be = decode_int(&short_prim_be, &mut bytes);
      let short_le = decode_int(&short_prim_le, &mut bytes);

      let int_be = decode_int(&int_prim_be, &mut bytes);
      let int_le = decode_int(&int_prim_le, &mut bytes);

      let long_be = decode_int(&long_prim_be, &mut bytes);
      let long_le = decode_int(&long_prim_le, &mut bytes);

      assert!(byte == Value::U8(0xAA));

      assert!(short_be == Value::U16(0x1122));
      assert!(short_le == Value::U16(0x4433));

      assert!(int_be == Value::U32(0x11223344));
      assert!(int_le == Value::U32(0x11223344));

      assert!(long_be == Value::U64(0x1122334455667788));
      assert!(long_le == Value::U64(0x1122334455667788));
    }
}

