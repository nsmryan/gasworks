
Possible directions:

implement limits

better way to describe packets, perhaps custom format

packet decoding can likely be faster
  builds hashmap instead of streaming results

support protocols

support different outputs- csv/tcp/udp/serial/file

support different inputs- csv/tcp/udp/serial/file



enum SecondaryHeaderPresent {
  NotPresent = 0,
  Present    = 1,
}

struct CcsdsHeader {
  version           : u8 : 3,
  type              : u8 : 1,
  secondary\_header : u8 : 1,
  apid              : u16 : 11,
} where {
  version = 0,
}
