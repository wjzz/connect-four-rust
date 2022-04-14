# result

0 - draw
10 black win
-10 white win

# results

| 4x4 |   0 |
| 5x5 |   0 |
| 4x5 |   0 |
| 5x5 |   0 |
| 6x5 |   0 |
| 5x6 |   0 |
| 6x6 | -10 |
| 4x6 | -10 |
| 4x7 |   0 |

# benchmarks (most results don't use the symmetry cutoff)

| size (row x cols) | result | nodes | time | notes
|-|-|-|-|-|
| 4x4 | draw | 131,060,741 | 5.6s   | simple minimax
| 4x4 | draw |  50,624,601 | 2.3s   | simple minimax with early win exit
| 4x4 | draw |     276,262 | 0.03s  | minimax w/ hashmap
| 4x4 | draw |      62,889 | 0.003s | alpha-beta
| 4x4 | draw |      30,996 | 0.003s | alpha-beta w/ move ordering

| 5x4 | draw | 2,594,019,977 | 2m 3s  | simple minimax with early win exit
| 5x4 | draw |     3,981,203 | 0.5s   | minimax w/ hashmap
| 5x4 | draw |     1,227,610 | 0.1s   | alpha-beta
| 5x4 | draw |       223,881 | 0.025s | alpha-beta w/ move ordering
| 5x4 | draw |        60,113 | 0.015s | alpha-beta w/ move ordering & hashmap

| 4x5 | draw | 11,776,485 | 1.3s  | minimax w/ hashmap
| 4x5 | draw | 18,026,614 | 0.97s | alpha-beta
| 4x5 | draw |  4,185,836 | 0.49s | alpha-beta w/ move ordering
| 4x5 | draw |    452,864 | 0.1s  | alpha-beta w/ move ordering & hashmap

| 5x5 | draw | 2,374,557,239 | 3m 14s | minimax w/ hashmap (3-10)
| 5x5 | draw |   312,660,988 |    39s | minimax w/ hashmap (3-12)
| 5x5 | draw | 2,135,732,339 | 2m 05s | alpha-beta
| 5x5 | draw |   111,610,966 |    13s | alpha-beta w/ move ordering
| 5x5 | draw |     7,891,967 |   1.2s | alpha-beta w/ move ordering & hashmap
| 5x5 | draw |     2,550,948 |   0.6s | alpha-beta w/ move ordering & hashmap (PC, MAX HASHMAP)

| 6x5 | draw |  1,801,601,736 | 3m 30s | alpha-beta w/ move ordering
| 6x5 | draw |     25,860,327 |     7s | alpha-beta w/ move ordering & hashmap
| 6x5 | draw |     17,739,258 |     5s | alpha-beta w/ move ordering & hashmap (PC, MAX HASHMAP)

| 5x6 | draw | 10,030,145,152 | 19m 31s | alpha-beta w/ move ordering
| 5x6 | draw |     47,382,760 |     14s | alpha-beta w/ move ordering & hashmap
| 5x6 | draw |     41,112,828 |     12s | alpha-beta w/ move ordering & hashmap (PC, MAX HASHMAP)

| 6x6 | white win | 9,591,023,814 | 27min 20s | alpha-beta w/ move ordering & hashmap (420mb)
| 6x6 | white win | 6,626,914,896 | 18min 44s | alpha-beta w/ move ordering & hashmap (1.6gb)
| 6x6 | white win | 1,088,699,620 |  7min 00s | alpha-beta w/ move ordering & hashmap (20gb)
| 6x6 | white win | 1,316,410,640 |  6min 40s | alpha-beta w/ move ordering & my hash & no flag (1.5gb)

| 6x7 | black win | 21,298,838,143 | 1h 56 min | alpha-beta w/ move ordering & hashmap (32gb)
| 6x7 | black win | 20,280,804,729 | 1h 40 min | alpha-beta w/ move ordering & my hash & no flag & symmetry (1.5gb)
| 6x7 | black win |  1,788,661,574 |    10 min | alpha-beta w/ better move ordering & my hash & no flag & symmetry (1.5gb)

| 4x6 | white win | 5,358,177 | 1.5s | alpha-beta w/ move ordering & hashmap (20gb)

| 4x7 | draw | 65,901,814 | 20s | alpha-beta w/ move ordering & hashmap (20gb)

| 4x8 | white win | 1,039,834,109 | 5min 37s | alpha-beta w/ move ordering & hashmap (32gb)

| 7x4 | draw | 1,005,078 | 8.5s | alpha-beta w/ move ordering & hashmap (32gb)

| 8x4 | draw |  6,848,615 | 10s | alpha-beta w/ move ordering & hashmap (32gb)
