extern crate heapsize;
extern crate gasworks;

use std::mem;

use gasworks::types::*;


fn main()
{
    println!("sizeof(ChoicePoints) = {}", mem::size_of::<ChoicePoints>());
    println!("sizeof(Endianness) = {}", mem::size_of::<Endianness>());
    println!("sizeof(IntSize) = {}", mem::size_of::<IntSize>());
    println!("sizeof(Signedness) = {}", mem::size_of::<Signedness>());
    println!("sizeof(FloatPrim) = {}", mem::size_of::<FloatPrim>());
    println!("sizeof(IntPrim) = {}", mem::size_of::<IntPrim>());
    println!("sizeof(BitPrim) = {}", mem::size_of::<BitPrim>());
    println!("sizeof(Enum) = {}", mem::size_of::<Enum>());
    println!("sizeof(Prim) = {}", mem::size_of::<Prim>());
    println!("sizeof(Item) = {}", mem::size_of::<Item>());
    println!("sizeof(LocPath) = {}", mem::size_of::<LocPath>());
    println!("sizeof(LocItem) = {}", mem::size_of::<LocItem>());
    println!("sizeof(LocLayout) = {}", mem::size_of::<LocLayout>());
    println!("sizeof(Layout) = {}", mem::size_of::<Layout>());
    println!("sizeof(ArrSize) = {}", mem::size_of::<ArrSize>());
    println!("sizeof(LayoutPacketDef) = {}", mem::size_of::<LayoutPacketDef>());
    println!("sizeof(LocPacketDef) = {}", mem::size_of::<LocPacketDef>());
    println!("sizeof(Packet) = {}", mem::size_of::<Packet>());
    println!("sizeof(Protocol<Layout>) = {}", mem::size_of::<Protocol<Layout>>());
    println!("sizeof(Protocol<LocItem>) = {}", mem::size_of::<Protocol<LocItem>>());
    println!("sizeof(LayoutMap) = {}", mem::size_of::<LayoutMap>());
    println!("sizeof(Value) = {}", mem::size_of::<Value>());
    println!("sizeof(Point) = {}", mem::size_of::<Point>());
    println!("sizeof(ValueMap) = {}", mem::size_of::<ValueMap>());
    println!("sizeof(ValueEntry) = {}", mem::size_of::<ValueEntry>());
}

