# Number of nodes depending on the ordering heuristic

| Number | Type | Node count (5x5) | Node count (6x5) [cutoff%] |
| - | - | - | - |
| Ord-1| Killer moves(2) | 11,068,952 |
| Ord0 | No ordering | 10,068,200 |
| Ord1 | Central moves first | 5,362,705 | 39,101,025 (61%) |
| Ord2 | win or #threes | 5,301,253 | 39,488,376 (80%) |
| Ord3 | Move can make line | 2,561,609 | 17,821,309 (85%) |
| Ord4 | Ord2 & Ord1 | 2,451,539 | 8,962,394 (84%) |
| Ord5 | Ord3 & Ord1 | 1,525,381 | 6,256,899 (86%) |
