benchmark:
	@cargo run --release -q -- --rollout 50000

perft:
	@cargo run --release -q -- --perft 4
