
Possible directions:

implement limits

better way to describe packets, perhaps custom format

packet decoding can likely be faster
  builds hashmap instead of streaming results

support protocols

support different outputs- csv/tcp/udp/serial/file

support different inputs- csv/tcp/udp/serial/file



Possible packet syntax:

packet DMP {
  ccsds\_header  : CcsdsHeader,
  fixed\_portion : DmpFixed,
  variable\_10Hz : Dmp10Hz,
  variable\_1Hz  : Dmp1Hz,
}

enum SecondaryHeaderPresent {
  NotPresent = 0,
  Present    = 1,
}

value DMP {
  ccsds\_header.type = DATA,
  ccsds\_header.secondary\_header = PRESENT,
  ccsds\_header.apid\_header = 411,
}

struct CcsdsHeader {
  version           : u8 : 3,
  type              : u8 : 1,
  secondary\_header : u8 : 1,
  apid              : u16 : 11,
} where {
  version = 0,
}

struct DmpFixed {
  dmp\_10Hz\_flag : u8
}

union Dmp10Hz {
  Dmp10Hz\_0,
  Dmp10Hz\_1,
  Dmp10Hz\_2,
  Dmp10Hz\_3,
  Dmp10Hz\_4,
}

struct Dmp10Hz {
  fields : u8
} where {
  dmp\_10Hz\_flag = 1;
}

struct Dmp1Hz {
}
