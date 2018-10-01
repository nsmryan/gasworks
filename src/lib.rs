#[cfg(feature = "profile")]extern crate flame;
extern crate bitreader;
extern crate bytes;
#[macro_use] extern crate serde;
extern crate byteorder;
extern crate ron;

use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::BTreeMap;
#[allow(unused_imports)]
use std::iter::Iterator;

use bitreader::BitReader;

#[allow(unused_imports)]
use bytes::{Bytes, Buf};

#[allow(unused_imports)]
use byteorder::{LittleEndian, BigEndian, ByteOrder};



#[allow(unused_imports)]
use std::io::{Cursor, Read};

pub mod types;
use types::*;

pub mod decode;
use decode::*;

pub mod csv;
// use csv::*;


pub fn decode_to_map(layout : &Layout, bytes : &mut Cursor<&[u8]>) -> ValueMap {
    let mut map = ValueMap::new(BTreeMap::new());

    let _ = decode_layout(layout, bytes, &mut map);

    map
}

pub fn decode_prim(prim : &Prim, bytes : &mut Cursor<&[u8]>) -> Value {
    let value : Value;

    #[cfg(feature = "profile")] flame::start("decode prim");
    match prim {
        Prim::Int(int_prim) => {
            #[cfg(feature = "profile")] flame::start("decode int");
            value = decode_int(int_prim, bytes);
            #[cfg(feature = "profile")] flame::end("decode int");
        },

        Prim::Float(float_type) => {
            #[cfg(feature = "profile")] flame::start("decode float");
            value = match float_type {
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
            };
            #[cfg(feature = "profile")] flame::end("decode float");
        },

        // NOTE add bytes back in when you understand how to avoid copying
        //Prim::Bytes(num_bytes) => {
        //    let buf = vec![0; num_bytes];
        //    
        //    bytes.read_exact(buf.as_mut_slice()).unwrap();

        //    Value::Bytes(buf.as_slice())
        //},

        Prim::Enum(Enum{map, int_prim}) => {
            #[cfg(feature = "profile")] flame::start("decode enum");
            // NOTE this doesn't ensure that the int_prim
            // decodes to a value in the map, and doesn't
            // enforce that the int_value.value doesn't loose precision
            let int_value = decode_int(int_prim, bytes);
            let int = int_value.value();
            // NOTE the use of to_string here may be wrong?
            value = Value::Enum(map.get(&int).unwrap().to_string(), int);
            #[cfg(feature = "profile")] flame::end("decode enum");
        },
    }
    #[cfg(feature = "profile")] flame::end("decode prim");

    value
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

fn decode_bits(bits : &BitPrim, bytes : &mut Cursor<&[u8]>, map : &mut ValueMap) {
    let BitPrim{entries, num_bytes} = bits;
    {
        let slice = bytes.get_mut();
        let mut reader = BitReader::new(slice);
        for (name, num_bits, int_prim) in entries.iter() {
            match int_prim.signedness {
                Signedness::Unsigned => {
                    match int_prim.size {
                      IntSize::Bits8 => {
                          let int_value = Value::U8(reader.read_u8(*num_bits as u8).unwrap());
                          map.value_map.insert(name.to_string(), ValueEntry::Leaf(int_value));
                      },
                      IntSize::Bits16 => {
                          let int_value = Value::U16(reader.read_u16(*num_bits as u8).unwrap());
                          map.value_map.insert(name.to_string(), ValueEntry::Leaf(int_value));
                      },
                      IntSize::Bits32 => {
                          let int_value = Value::U32(reader.read_u32(*num_bits as u8).unwrap());
                          map.value_map.insert(name.to_string(), ValueEntry::Leaf(int_value));
                      },
                      IntSize::Bits64 => {
                          let int_value = Value::U64(reader.read_u64(*num_bits as u8).unwrap());
                          map.value_map.insert(name.to_string(), ValueEntry::Leaf(int_value));
                      },
                    }
                },

                Signedness::Signed => {
                    match int_prim.size {
                      IntSize::Bits8 => {
                          let int_value = Value::I8(reader.read_i8(*num_bits as u8).unwrap());
                          map.value_map.insert(name.to_string(), ValueEntry::Leaf(int_value));
                      },
                      IntSize::Bits16 => {
                          let int_value = Value::I16(reader.read_i16(*num_bits as u8).unwrap());
                          map.value_map.insert(name.to_string(), ValueEntry::Leaf(int_value));
                      },
                      IntSize::Bits32 => {
                          let int_value = Value::I32(reader.read_i32(*num_bits as u8).unwrap());
                          map.value_map.insert(name.to_string(), ValueEntry::Leaf(int_value));
                      },
                      IntSize::Bits64 => {
                          let int_value = Value::I64(reader.read_i64(*num_bits as u8).unwrap());
                          map.value_map.insert(name.to_string(), ValueEntry::Leaf(int_value));
                      },
                    }
                },
            }
        }
    }

    let current_position = bytes.position();
    bytes.set_position(current_position +
                       num_bytes);
}

fn decode_layout(layout : &Layout, bytes : &mut Cursor<&[u8]>, map : &mut ValueMap) {
    
    match layout {
        Layout::Prim(item) => {
            let value = decode_prim(&item.typ, bytes);
            map.value_map.insert(item.name.to_string(), ValueEntry::Leaf(value));
        },

        Layout::Seq(_, layouts) => {
            for layout in layouts.iter() {
                decode_layout(layout, bytes, map);
            }
        },

        Layout::All(name, layouts) => {
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

        Layout::Array(name, size, layout) => {
            let mut array = Vec::new();

            for _ in 0 .. *size {
                let mut value_map = ValueMap::new(BTreeMap::new());
                decode_layout(layout, bytes, &mut value_map);
                array.push(value_map);
            }

            map.value_map.insert(name.to_string(), ValueEntry::Array(array));
        }

        // NOTE - Bit fields currently do not support endianness choice
        //        bitreverse crate could help with this.
        Layout::Bits(bits) => {
            decode_bits(bits, bytes, map);
        }
    }
}

pub fn decode_loc_layout(loc_layout : &LocLayout, bytes : &mut Cursor<&[u8]>) -> Vec<Point> {
    loc_layout.loc_items.iter()
                        .map(|loc_item| {
                            decode_loc_item(loc_item, bytes)
                        })
                        .collect()
}

pub fn decode_loc_item(loc_item : &LocItem, bytes : &mut Cursor<&[u8]>) -> Point {
    bytes.set_position(loc_item.loc);
    Point::new(loc_item.name.last().unwrap().clone(), decode_prim(&loc_item.typ, bytes))
}

pub fn decode_layoutpacket(layout_packet : &LayoutPacketDef,
                           bytes         : &mut Cursor<&[u8]>) -> ValueMap {
    let mut map = ValueMap::new(BTreeMap::new());

    decode_layoutpacket_helper(layout_packet, bytes, &mut map);

    map
}

pub fn decode_layoutpacket_helper(layout_packet : &LayoutPacketDef,
                                  bytes         : &mut Cursor<&[u8]>,
                                  map           : &mut ValueMap) {
    match layout_packet {
        PacketDef::Seq(packets) => {
            for packet in packets {
                decode_layoutpacket_helper(packet, bytes, map);
            }
        },

        PacketDef::Subcom(item, subcom) => {
            let entry = map.value_map[&item.name].clone();

            match entry {
               ValueEntry::Leaf(value) => {
                  for (item_key, packet) in subcom {
                      let subcom_entry = &map.value_map[&item_key.name].clone();
                      match subcom_entry {
                         ValueEntry::Leaf(subcom_value) => {
                             if value == *subcom_value {
                                 decode_layoutpacket_helper(packet, bytes, map);
                                 break;
                             }

                         }

                         _ => (),
                      }
                  }
               },

               // NOTE if the name refers to something else then a value, it is skipped
               _ => ()
            }
        },

        PacketDef::Array(size, packet) => {
            let num_elements : usize;
            match size {
                ArrSize::Fixed(num) => {
                    num_elements = *num;
                }

                ArrSize::Var(name) => {
                    // TODO this should search the full map recursively.
                    // an optimization would be to preprocess the packet and keep track of
                    // a map of names that need to be used like this.
                    num_elements = map.lookup(&name.to_string()).unwrap().value() as usize;
                }
            }
            for _ in 0..num_elements {
              decode_layoutpacket_helper(packet, bytes, map);
            }
        }

        PacketDef::Leaf(item) => {
            let prim = decode_prim(&item.typ, bytes);
            #[cfg(feature = "profile")] flame::start("insert prim");
            map.value_map.insert(item.name.clone(),
                                 ValueEntry::Leaf(prim));
            #[cfg(feature = "profile")] flame::end("insert prim");
        },
    }
}

pub fn identify_locpacket(packet : &LocPacketDef, bytes : &mut Cursor<&[u8]>) -> LocLayout
{
    let locs = Vec::new();

    let mut loc_layout = LocLayout{ loc_items : locs};

    identify_locpacket_helper(packet, bytes, &mut loc_layout);

    loc_layout
}

fn identify_locpacket_helper(packet : &LocPacketDef, 
                             bytes : &mut Cursor<&[u8]>,
                             loc_layout : &mut LocLayout) 
{
    match packet {
        PacketDef::Seq(packets) => {
            for packet in packets {
                identify_locpacket_helper(packet, bytes, loc_layout);
            }
        },

        PacketDef::Subcom(item, subcom) => {
            // NOTE we are decoding items here and throwing them away. the assumption is that
            // we don't decode many items, and don't need to keep our work.
            // we re-decode items even if they are used in other iterations of this loop!
            let value = decode_loc_item(&item, bytes);

            for (item_key, packet_value) in subcom {
                let item_value = decode_loc_item(item_key, bytes);

                if value == item_value {
                    identify_locpacket_helper(packet_value, bytes, loc_layout);
                    break;
                }
            }
        },

        // NOTE an optimization here would be to use a hashmap, or
        // to keep only values used in decisions, determined beforehand.
        PacketDef::Array(size, packet) => {
            let mut num_elements : usize = 0;
            match size {
                ArrSize::Fixed(num) =>
                    num_elements = *num,

                ArrSize::Var(name)  => {
                    for elem in loc_layout.loc_items.iter() {
                        // NOTE matches the first item with the name
                        // might need better matching
                        if name == &elem.name[elem.name.len() - 1] {
                            let point = decode_loc_item(&elem, bytes);
                            num_elements = point.val.value() as usize;
                            break;
                        }
                    }
                }
            }

            // NOTE If Var, and name is not found, the array will be skipped
            for _ in 0..num_elements {
                identify_locpacket_helper(packet, bytes, loc_layout);
            }
        }

        PacketDef::Leaf(layer_loc_layout) => {
            // NOTE use of clone
            loc_layout.loc_items.push(layer_loc_layout.clone());
        },
    }
}

// Find all locations in a packet where a choice is made based
// on a value in the packet.
// This results in a map which initially maps each telemetry point
// name to None, but can be updated with particular values when
// decoding a particular packet.
pub fn choice_points(packet : &LayoutPacketDef) -> ChoicePoints {
    // NOTE consider passing a single hashmap around instead
    // of using 'extend' on subtree's choice points.
    let mut map = HashMap::new();

    match packet {
        PacketDef::Seq(packets) => {
            for packet in packets {
                map.extend(choice_points(packet))
            }
        },

        PacketDef::Subcom(item, subcom) => {
            map.insert(item.name.clone(), None);
            for pair in subcom {
                map.extend(choice_points(&pair.1))
            }
        },

        PacketDef::Array(size, packet) => {
            match size {
                ArrSize::Var(name)  => {
                    map.insert(name.clone(), None);
                },

                _ => (),
            }
            map.extend(choice_points(packet));
        }

        PacketDef::Leaf(_) => {
            ()
        },
    } 

    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_layout() {
      let bit_entries =
          vec![("bits0".to_string(), 4,  IntPrim::u8_be()),
               ("bits1".to_string(), 12, IntPrim::u16_be()),
               ("bits2".to_string(), 2,  IntPrim::u8_be()),
               ("bits3".to_string(), 14, IntPrim::u32_be())];
      let bits_layout = Layout::Bits(BitPrim{entries : bit_entries, num_bytes : 4});

      let all_vec = vec![Layout::Prim(Item::new("all0".to_string(), Prim::Int(IntPrim::u8_be()))),
                         Layout::Prim(Item::new("all1".to_string(), Prim::Int(IntPrim::u32_be()))),
                         Layout::Prim(Item::new("all2".to_string(), Prim::Int(IntPrim::u8_be())))];
      let all_layout = Layout::All("all".to_string(), all_vec);

      let prim_layout = Layout::Prim(Item::new("prim0".to_string(), Prim::Int(IntPrim::u8_be())));

      let v = vec![0x12, 0x34, 0x56, 0x78,
                   0x12, 0x34, 0x56, 0x78,
                   0xAA
                  ];
      let mut bytes = Cursor::new(v.as_slice());

      let layout = Layout::Seq("seq".to_string(), vec![bits_layout, all_layout , prim_layout]);

      let value_map = decode_to_map(&layout, &mut bytes);

      let value_bits0 = value_map.value_map.get(&"bits0".to_string()).unwrap();
      assert!(*value_bits0 == ValueEntry::Leaf(Value::U8(0x01)));

      let value_bits1 = value_map.value_map.get(&"bits1".to_string()).unwrap();
      assert!(*value_bits1 == ValueEntry::Leaf(Value::U16(0x0234)));

      let value_bits2 = value_map.value_map.get(&"bits2".to_string()).unwrap();
      assert!(*value_bits2 == ValueEntry::Leaf(Value::U8(0x01)));

      let value_bits3 = value_map.value_map.get(&"bits3".to_string()).unwrap();
      assert!(*value_bits3 == ValueEntry::Leaf(Value::U32(0x00001678)));

      let value_all0 = value_map.value_map.get(&"all0".to_string()).unwrap();
      assert!(*value_all0 == ValueEntry::Leaf(Value::U8(0x12)));

      let value_all1 = value_map.value_map.get(&"all1".to_string()).unwrap();
      assert!(*value_all1 == ValueEntry::Leaf(Value::U32(0x12345678)));

      let value_all2 = value_map.value_map.get(&"all2".to_string()).unwrap();
      assert!(*value_all0 == ValueEntry::Leaf(Value::U8(0x12)));

      let value_prim0 = value_map.value_map.get(&"prim0".to_string()).unwrap();
      assert!(*value_prim0 == ValueEntry::Leaf(Value::U8(0xAA)));
    }

    #[test]
    fn test_decode_prim() {
      let byte_prim = Prim::Int(IntPrim::new(IntSize::Bits8, Signedness::Unsigned, Endianness::BigEndian));

      let float32_be = Prim::Float(FloatPrim::F32(Endianness::BigEndian));
      let float64_be = Prim::Float(FloatPrim::F64(Endianness::BigEndian));

      let float32_le = Prim::Float(FloatPrim::F32(Endianness::LittleEndian));
      let float64_le = Prim::Float(FloatPrim::F64(Endianness::LittleEndian));

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
      let enum_value_one  = decode_prim(&enum_prim, &mut bytes);
      let enum_value_two  = decode_prim(&enum_prim, &mut bytes);
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

