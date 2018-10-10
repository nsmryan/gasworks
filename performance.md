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
second run, 5.003 seconds- invalid result- not actually decoding.
16.74 MB/second

Eighth Attempt:
release build, writing in separate thread, 13.748 seconds.

Ninth Attempt:
release build, rayon threading, 10 per channel, 24 seconds

Tenth Attempt:
release build, rayon threading, 1 per channel, 26 seconds

Eleventh Attempt:
release build, rayon threading, 1 per channel, 24 seconds

12th Attempt:
release build, multithreading, 4 threads, 6.5 seconds

13th Attempt:
release, 10 packet, 10 line
threads, seconds
1, 19.78
2, 11.67
3,  9.07
4,  7.87
5,  6.81
6,  6.31
7,  6.489
8,  7.513

2000 line, 2000 packet, 8 threads, 5 seconds
18 MBps
