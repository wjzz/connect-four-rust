# Benchmarks

## Array representation

nodes after 1 moves =           225 [elapsed: 0.000s] [speed:   0M nps]
nodes after 2 moves =        50,400 [elapsed: 0.000s] [speed:  50M nps]
nodes after 3 moves =    11,239,200 [elapsed: 0.094s] [speed: 119M nps]
nodes after 4 moves = 2,495,102,400 [elapsed:  21.7s] [speed: 115M nps]

5 moves => expected 15^2 * 4 moves ==~ 82 mins

# After algorithm fix (depth=1 ==> return move length)

nodes after 3 moves =    11,239,200 [elapsed: 0.041s] [speed: 274M nps]
nodes after 4 moves = 2,495,102,400 [elapsed: 8.1s]   [speed: 305M nps]

# After algorithm fix (depth=1 => only count moves)

nodes after 3 moves =    11,239,200 [elapsed: 0.014s] [speed: 802M nps]
nodes after 4 moves = 2,495,102,400 [elapsed: 2.8s]   [speed: 877M nps]

## Bitboards

nodes after 3 moves =    11,239,200 [elapsed:  0.1s] [speed: 119M nps]
nodes after 4 moves = 2,495,102,400 [elapsed: 21.1s] [speed: 117M nps]

# After algorithm fix (depth=1 ==> return move length)

nodes after 3 moves =    11,239,200 [elapsed:  0.023s] [speed: 488M nps]
nodes after 4 moves = 2,495,102,400 [elapsed:  5.4s]   [speed: 486M nps]

# After algorithm fix (depth=1 => only count moves)

nodes after 3 moves =    11,239,200 [elapsed: 0.009s] [speed: 1,248M nps]
nodes after 4 moves = 2,495,102,400 [elapsed: 2.2s]   [speed: 1,109M nps]

# After algorithm fix (depth=1 => only count moves, using the count_ones function)

nodes after 3 moves =      11,239,200 [elapsed: 0s]   [speed: 11,239M nps]
nodes after 4 moves =   2,495,102,400 [elapsed: 0.2s] [speed: 13,560M nps]
nodes after 5 moves = 551,417,630,400 [elapsed: 42s]  [speed: 13,187M nps]


# Ideas

- [ ] Search in parallel
- [ ] Use transposition tables
