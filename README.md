#GasWorks


This project is a packet handling tool which is designed for
the needs of embedded system development. It is intended to
provide features that are commonly needed for developing
embedded software systems, particularly in the Aerospace domain.


This software is very early in development and provides essentially
none of the following intended features:

* Command and Telemetry decoding and streaming
* Support for a variety of packet formats, including subcommutated data,
  bit fields, buffers of bytes, arrays, variable size fields,
  multiple decoding of the same data, as well as any others that turn
  out to be necessary to describe common packet formats.
* Derived parameters- parameters that are not explicitly in a packet, but
  are calculated based on fields in the packet and other derived parameters.
* Engineering conversions- things like linear fits, polynominals, and lookup
  tables would be useful for decoding and reporting results, as well
  as limit monitoring.
* Limit monitoring with persistence
* Packet indexing for faster retrievals- ideally this would include recording
  which packet definitions where used (perhaps with a hash), so we can decode
  even after definitions change.
* Command and telemetry creation- it would be nice to be able to build
  commands and telemetry somehow to simulate and stimulate different systems.
  This would make a nice testing tool.



In the ideal case, this tool would become a kind of packet swiss army knife
for embedded systems packet handling, providing the most common needs for
routing, decoding, monitoring, storing, and retriving packet data outside
of a database. In the worst case, I hope it at least can decode packets from
a description, and that the description is versatile enough to support the
range of formats I deal with in my day job.


## Things That can be Done
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

create an iterator for packet items for layout and loc packets.
  this would prevent us from building hash maps and then throwing
  them away all the time.


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
