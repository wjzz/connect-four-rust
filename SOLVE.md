# result

0 - draw
10 black win
-10 white win

# results

| size (row x cols) | result | nodes | time | notes
|-|-|-|-|-|
| 4x4 | draw | 131,060,741 | 5.6s  | simple minimax
| 4x4 | draw |  50,624,601 | 2.3s  | simple minimax with early win exit
| 4x4 | draw |     276,262 | 0.03s | minimax w/ hashmap

| 5x4 | draw | 2,594,019,977 | 2m 3s | simple minimax with early win exit
| 5x4 | draw |     3,981,203 | 0.5s  | minimax w/ hashmap

| 4x5 | draw | 11,776,485 | 1.3s | minimax w/ hashmap

| 5x5 | draw | 2,374,557,239 | 3m 14s | minimax w/ hashmap (3-10)
| 5x5 | draw |   312,660,988 | 39s    | minimax w/ hashmap (3-12)
