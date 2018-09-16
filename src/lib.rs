
#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::BTreeMap;
#[allow(unused_imports)]
use std::iter::Iterator;

extern crate bitreader;
use bitreader::BitReader;

extern crate bytes;
#[allow(unused_imports)]
use bytes::{Bytes, Buf};

extern crate byteorder;
#[allow(unused_imports)]
use byteorder::{LittleEndian, BigEndian, ByteOrder};

extern crate ron;

#[macro_use]
extern crate serde;

#[allow(unused_imports)]
use std::io::{Cursor, Read};

pub mod types;
use types::*;

pub mod decode;
use decode::*;

pub mod csv;
// use csv::*;


pub fn identify_locpacket(packet : &LocPacket, bytes : &mut Cursor<&[u8]>) -> LocLayout
{
    let locs = Vec::new();

    let mut loc_layout = LocLayout{ loc_items : locs};

    identify_locpacket_helper(packet, bytes, &mut loc_layout);

    loc_layout
}

fn identify_locpacket_helper(packet : &LocPacket, 
                             bytes : &mut Cursor<&[u8]>,
                             loc_layout : &mut LocLayout) 
{
    match packet {
        Packet::Seq(packets) => {
            for packet in packets {
                identify_locpacket_helper(packet, bytes, loc_layout);
            }
        },

        Packet::Subcom(item, subcom) => {
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

        Packet::Leaf(layer_loc_layout) => {
            // NOTE use of clone
            loc_layout.loc_items.push(layer_loc_layout.clone());
        },
    }
}

pub fn choice_points(packet : &LayoutPacket) -> ChoicePoints {
    let mut map = HashMap::new();

    match packet {
        Packet::Seq(packets) => {
            for packet in packets {
                map.extend(choice_points(packet))
            }
        },

        Packet::Subcom(item, subcom) => {
            map.insert(item.name.clone(), None);
            for pair in subcom {
                map.extend(choice_points(&pair.1))
            }
        },

        Packet::Leaf(layer_loc_layout) => {
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
      let all_layout = Layout::All(all_vec);

      let prim_layout = Layout::Prim(Item::new("prim0".to_string(), Prim::Int(IntPrim::u8_be())));

      let v = vec![0x12, 0x34, 0x56, 0x78,
                   0x12, 0x34, 0x56, 0x78,
                   0xAA
                  ];
      let mut bytes = Cursor::new(v.as_slice());

      let layout = Layout::Seq(vec![bits_layout, all_layout , prim_layout]);

      let value_map = decode_to_map(&layout, &mut bytes);

      let value_bits0 = value_map.get(&"bits0".to_string()).unwrap();
      assert!(*value_bits0 == Value::U8(0x01));

      let value_bits1 = value_map.get(&"bits1".to_string()).unwrap();
      assert!(*value_bits1 == Value::U16(0x0234));

      let value_bits2 = value_map.get(&"bits2".to_string()).unwrap();
      assert!(*value_bits2 == Value::U8(0x01));

      let value_bits3 = value_map.get(&"bits3".to_string()).unwrap();
      assert!(*value_bits3 == Value::U32(0x00001678));

      let value_all0 = value_map.get(&"all0".to_string()).unwrap();
      assert!(*value_all0 == Value::U8(0x12));

      let value_all1 = value_map.get(&"all1".to_string()).unwrap();
      assert!(*value_all1 == Value::U32(0x12345678));

      let value_all2 = value_map.get(&"all2".to_string()).unwrap();
      assert!(*value_all0 == Value::U8(0x12));

      let value_prim0 = value_map.get(&"prim0".to_string()).unwrap();
      assert!(*value_prim0 == Value::U8(0xAA));
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

