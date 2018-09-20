
Things to Consider:
implement limits and limit checking

better way to describe packets for user
  custom format
  cosmos config parsing
  XTCE

packet decoding can likely be faster
  builds hashmap instead of streaming results

support protocols
  need to decide on what packet we are looking at
  could optimize by decoding minimum amount necessary

support different outputs- csv/tcp/udp/serial/file

support different inputs- csv/tcp/udp/serial/file

support LocPackets- need way to handle the situation
  where the location and presense of an item depends on the
  packet contents

look into extracting a subset of telemetry by only decoding
  desired items and items necessary to determine the location
  and presense of desired items


possible custom format:
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
