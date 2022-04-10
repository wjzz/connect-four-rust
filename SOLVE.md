# result

0 - draw
10 black win
-10 white win

# results

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
| 5x5 | draw |   312,660,988 | 39s    | minimax w/ hashmap (3-12)
| 5x5 | draw | 2,135,732,339 | 2m 5s  | alpha-beta
| 5x5 | draw |   111,610,966 | 13s    | alpha-beta w/ move ordering
| 5x5 | draw |     7,891,967 | 1.2s   | alpha-beta w/ move ordering & hashmap

| 6x5 | draw |  1,801,601,736 | 3m 30s | alpha-beta w/ move ordering
| 6x5 | draw |     25,860,327 |     7s | alpha-beta w/ move ordering & hashmap

| 5x6 | draw | 10,030,145,152 | 19m 31s | alpha-beta w/ move ordering
| 5x6 | draw |     47,382,760 |     14s | alpha-beta w/ move ordering & hashmap

## move ordering improvements

depth = 26 | result =      0 | nodes = 19,625,891   | [elapsed: 2375] [speed: 8,263K nps]
depth = 26 | result =      0 | nodes = 9,841,261    | [elapsed: 1479] [speed: 6,653K nps]
depth = 26 | result =      0 | nodes = 7,891,967    | [elapsed: 1231] [speed: 6,411K nps]