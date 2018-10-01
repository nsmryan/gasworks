First Attempt:
debug build, took several minutes, crashed before finishing
resulted in >700 MB csv from 93 MB binary VN200 data.

Second Attempt:
debug build, 1 minute, 24 seconds, 184 MB csv file
1.08 MB/second

C version took 9.968 second, then 16 seconds. csv is 148 MB
5.66 MB/second for 16 seconds

Third Attempt:
release build, 16 seconds. 184 MB of csv.

c version took 9.77 seconds on SD drive
9.25 MB/second

Fourth Attempt:
debug build, mmapped file, 1 minutes 24 seconds
no change from regular file use

Fifth Attempt:
release build, mmapped file, 16.626 seconds
no change from regular file use

Sixth Attempt:
release build, reused csv record strings, 7.178 seconds
12.679 MB/second

Seventh Attempt:
release build, fnv hashmap, 5.436 seconds
second run, 5.003 seconds
16.74 MB/second
