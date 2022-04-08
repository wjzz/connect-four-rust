# Light rollouts

## ArrayPosition v1.

```
performed 5000 rollouts [elapsed: 0.516s] [speed: 9K rps]
```

We can do around 10k rollouts per second

## Bitboard benchmark

performed 5000 rollouts [elapsed: 0.459s] [speed: 10K rps]

### with preallocated buffer

performed 5000 rollouts [elapsed: 0.379s] [speed: 13K rps]

## Time comparison: no randomization

```
performed 5000 rollouts [elapsed: 0.233] [speed: 21K rps]
```

## Using the move list vector

### Array

performed 50000 rollouts [elapsed: 2.0s] [speed: 25K rps]

### Bitboard

performed 50000 rollouts [elapsed: 3.5s] [speed: 14K rps]
