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

use std::cmp;

#[allow(unused_imports)]
use self::bytes::{Bytes, Buf};

use prim::*;
use types::*;
use loclayout::*;


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
    let mut names: HashSet<&Name> = HashSet::new();

    match self {
      Layout::Prim(Item{name, typ: _}) => {
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

    LocLayout { loc_items: loc_items }
  }

  pub fn locate_loc(&self,
                    loc_items: &mut Vec<LocItem>,
                    path: &LocPath,
                    loc: &mut Loc) {
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

