## Benchmarks

nodes after 1 moves =           225 [elapsed: 0.000s] [speed:   0M nps]
nodes after 2 moves =        50,400 [elapsed: 0.000s] [speed:  50M nps]
nodes after 3 moves =    11,239,200 [elapsed: 0.094s] [speed: 119M nps]
nodes after 4 moves = 2,495,102,400 [elapsed:  21.7s] [speed: 115M nps]

5 moves => expected 15^2 * 4 moves ==~ 82 mins

## Ideas

- [ ] Search in parallel
- [ ] Use transposition tables

## Bitboard benchmarks

nodes after 3 moves =    11,239,200 [elapsed: 0.146s] [speed: 76M nps]
nodes after 4 moves = 2,495,102,400 [elapsed:  32.7s] [speed: 76M nps]
