benchmark:
	@cargo run --release -q -- --rollout 1000000

perft:
	@cargo run --release -q -- --perft 4

play:
	@cargo run --release -q -- --play

solve:
	@cargo run --release -q -- --solve
