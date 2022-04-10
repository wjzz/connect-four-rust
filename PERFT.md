# 6x7

# version 1

nodes after 1 moves = 7            [elapsed:    1] [speed:   0M nps]
nodes after 2 moves = 49           [elapsed:    1] [speed:   0M nps]
nodes after 3 moves = 343          [elapsed:    1] [speed:   0M nps]
nodes after 4 moves = 2,401        [elapsed:    1] [speed:   2M nps]
nodes after 5 moves = 16,807       [elapsed:    1] [speed:  16M nps]
nodes after 6 moves = 117,649      [elapsed:    1] [speed: 117M nps]
nodes after 7 moves = 823,536      [elapsed:    2] [speed: 411M nps]
nodes after 8 moves = 5,673,234    [elapsed:   50] [speed: 113M nps]
nodes after 9 moves = 39,394,572   [elapsed:  425] [speed:  92M nps]
nodes after 10 moves = 268,031,646 [elapsed: 2891] [speed:  92M nps]

# 4x4

nodes after 1 moves = 4 [elapsed: 1] [speed: 0M nps]
nodes after 2 moves = 16 [elapsed: 1] [speed: 0M nps]
nodes after 3 moves = 64 [elapsed: 1] [speed: 0M nps]
nodes after 4 moves = 256 [elapsed: 1] [speed: 0M nps]
nodes after 5 moves = 1,020 [elapsed: 1] [speed: 1M nps]
nodes after 6 moves = 4,020 [elapsed: 1] [speed: 4M nps]
nodes after 7 moves = 15,540 [elapsed: 1] [speed: 15M nps]
nodes after 8 moves = 57,504 [elapsed: 1] [speed: 57M nps]
nodes after 9 moves = 206,904 [elapsed: 2] [speed: 103M nps]
nodes after 10 moves = 690,504 [elapsed: 9] [speed: 76M nps]
nodes after 11 moves = 2,160,504 [elapsed: 31] [speed: 69M nps]
nodes after 12 moves = 5,992,096 [elapsed: 105] [speed: 57M nps]
nodes after 13 moves = 14,712,024 [elapsed: 319] [speed: 46M nps]
nodes after 14 moves = 28,850,920 [elapsed: 865] [speed: 33M nps]
nodes after 15 moves = 42,756,080 [elapsed: 1946] [speed: 21M nps]
nodes after 16 moves = 35,613,284 [elapsed: 3725] [speed: 9M nps]

# 5x5

## naive is_win

nodes after 2 moves = 25 [elapsed: 1] [speed: 0M nps]
nodes after 3 moves = 125 [elapsed: 1] [speed: 0M nps]
nodes after 4 moves = 625 [elapsed: 1] [speed: 0M nps]
nodes after 5 moves = 3,125 [elapsed: 1] [speed: 3M nps]
nodes after 6 moves = 15,620 [elapsed: 1] [speed: 15M nps]
nodes after 7 moves = 77,980 [elapsed: 1] [speed: 77M nps]
nodes after 8 moves = 380,860 [elapsed: 4] [speed: 95M nps]
nodes after 9 moves = 1,874,080 [elapsed: 23] [speed: 81M nps]
nodes after 10 moves = 8,945,804 [elapsed: 118] [speed: 75M nps]
nodes after 11 moves = 42,776,068 [elapsed: 561] [speed: 76M nps]
nodes after 12 moves = 196,511,556 [elapsed: 2689] [speed: 73M nps]
nodes after 13 moves = 893,969,970 [elapsed: 12244] [speed: 73M nps]
nodes after 14 moves = 3,863,348,736 [elapsed: 55687] [speed: 69M nps]

## is_win with precomputed lines

...
nodes after 11 moves = 42,776,068 [elapsed: 837] [speed: 51M nps]
nodes after 12 moves = 196,511,556 [elapsed: 4009] [speed: 49M nps]
nodes after 13 moves = 893,969,970 [elapsed: 18304] [speed: 48M nps]
nodes after 14 moves = 3,863,348,736 [elapsed: 83214] [speed: 46M nps]

=> it seems slower -> maybe because the board is still small?

## is_win with last move and precomputed lines

nodes after 11 moves = 42,776,068 [elapsed: 596] [speed: 71M nps]
nodes after 12 moves = 196,511,556 [elapsed: 2997] [speed: 65M nps]
nodes after 13 moves = 893,969,970 [elapsed: 13579] [speed: 65M nps]
nodes after 14 moves = 3,863,348,736 [elapsed: 58362] [speed: 66M nps]

