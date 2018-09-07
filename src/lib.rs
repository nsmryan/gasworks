
use std::collections::HashSet;
use std::collections::HashMap;

extern crate bytes;
use bytes::{Bytes, Buf};

extern crate byteorder;
use byteorder::{LittleEndian, BigEndian, ByteOrder};

use std::io::{Cursor, Read};

mod types;
use types::*;


//fn decode(layout : Layout, bytes : Bytes) -> ValueMap {
//    let mut map = HashMap::new();
//
//    let _ = decode_at(&layout, &bytes, 0, &mut map);
//
//    map
//}

fn decode_prim(prim : Prim, bytes : &mut Cursor<&[u8]>, loc : Loc) -> Value {
    match prim {
        Prim::Int(int_prim) => {
            decode_int(int_prim, bytes)
        },

        Prim::Float(floatType) => {
            match floatType {
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

        Prim::Bits(num_bytes) => {
            unimplemented!()
        },

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

fn decode_int(int_prim : IntPrim, bytes : &mut Cursor<&[u8]>) -> Value {
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
                        IntSize::Bits16 => Value::U16(bytes.get_u16_be()),
                        IntSize::Bits32 => Value::U32(bytes.get_u32_be()),
                        IntSize::Bits64 => Value::U64(bytes.get_u64_be()),
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

// NOTE add decode_at back in, resulting in a HashMap<Name, Value>
//fn decode_at(layout : &Layout, bytes : Bytes, loc : Loc, map : &mut ValueMap) -> Value {
//    use types::Layout::*;
//    match layout {
//        Prim(prim) => {
//            decode_prim(prim, bytes, loc)
//        },
//
//        Seq(layouts) => {
//            let mut loc = loc;
//            for layout in layouts.iter() {
//                let new_loc = decode_at(layout, bytes, loc, map);
//                loc = new_loc;
//            }
//
//            loc
//        },
//
//        All(layouts) => {
//            let mut max_loc = loc;
//            for layout in layouts.iter() {
//                let new_loc = decode_at(layout, bytes, loc, map);
//                if new_loc > max_loc {
//                    max_loc = new_loc;
//                }
//            }
//
//            max_loc
//        },
//    }
//}


