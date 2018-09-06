
use std::collections::HashSet;
use std::collections::HashMap;

use bytes::*;

mod types;
use types::*;


fn decode(layout : Layout, bytes : Bytes) -> ValueMap {
    let mut map = HashMap::new();

    let _ = decode_at(&layout, &bytes, 0, &mut map);

    map
}

fn decode_prim(prim : Prim, bytes : Bytes, loc : Loc) -> Value {
    match prim {
        Int(int_prim) => {
            decode_int(int_prim, bytes, loc)
        },

        Float(floatType) => {
            match floatType {
                F32(endianness) => {
                    match endianness {
                        BigEndian   => BigEndian::read_f32(bytes.take(std::mem::size_of::<f32>())),
                        LitleEndian => LittleEndian::read_f32(bytes.take(std::mem::size_of::<f32>())),
                    },

                F64(endianness) => {
                    match endianness {
                        BigEndian   => BigEndian::read_f64(bytes.take(std::mem::size_of::<f64>())),
                        LitleEndian => LittleEndian::read_f64(bytes.take(std::mem::size_of::<f64>())),
                    },
            }
        },

        Bytes(num_bytes) => {
            Value::Bytes(bytes.take(num_bytes))
        },

        Bits(num_bytes) => {
            unimplemented!()
        },

        Enum(Enum{map, int_prim}) => {
            // NOTE this doesn't ensure that the int_prim
            // decodes to a value in the map, and doesn't
            // enforce that the int_value.value doesn't loose precision
            let int_value = decode_int(int_prim, bytes);
            let int = int_value.value;
            Value::Enum(map.get(int)?, int)
        },
    }
}

fn decode_int(int_prim, bytes) -> Value {
    let Int(IntPrim{size, signedness, endianness}) = int_prim;

    match endianness {
        BigEndian => {
            match signedness {
                Unsigned => {
                    match size {
                        Bits8  => Value::U8(bytes.get_u8()),
                        Bits16 => Value::U16(bytes.get_u16()),
                        Bits32 => Value::U32(bytes.get_u32()),
                        Bits64 => Value::U64(bytes.get_u64()),
                    }
                },

                Signed => {
                    match size {
                        Bits8  => Value::I8(bytes.get_i8()),
                        Bits16 => Value::I16(bytes.get_i16()),
                        Bits32 => Value::I32(bytes.get_i32()),
                        Bits64 => Value::I64(bytes.get_i64()),
                    }
                },
            }
        }

        LittleEndian => {
            match signedness {
                Unsigned => {
                    match size {
                        Bits8  => Value::I8(bytes.get_u8()),
                        Bits16 => Value::I16(bytes.get_u16_le()),
                        Bits32 => Value::I32(bytes.get_u32_le()),
                        Bits64 => Value::I64(bytes.get_u64_le()),
                    }
                },

                Signed => {
                    match size {
                        Bits8  => Value::I8(bytes.get_i8()),
                        Bits16 => Value::I16(bytes.get_i16_le()),
                        Bits32 => Value::I32(bytes.get_i32_le()),
                        Bits64 => Value::I64(bytes.get_i64_le()),
                    }
                },
            }
        }
    }
}

fn decode_at(layout : &Layout, bytes : Bytes, loc : Loc, map : &mut ValueMap) -> Loc {
    use types::Layout::*;
    match layout {
        Prim(prim) => {
            decode_prim(prim, bytes, loc);
        },

        Seq(layouts) => {
            let mut loc = loc;
            for layout in layouts.iter() {
                let new_loc = decode_at(layout, bytes, loc, map);
                loc = new_loc;
            }

            loc
        },

        All(layouts) => {
            let mut max_loc = loc;
            for layout in layouts.iter() {
                let new_loc = decode_at(layout, bytes, loc, map);
                if new_loc > max_loc {
                    max_loc = new_loc;
                }
            }

            max_loc
        },
    }
}


