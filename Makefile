benchmark:
	@cargo run --release -q -- --rollout 1000000

perft:
	@cargo run --release -q -- --perft 11

play:
	@cargo run --release -q -- --play

solve:
	@/usr/bin/time -f "%M kBs of ram" cargo run --release -q -- --solve

test:
	@python3 -m pytest -s tests.py

quiet:
	@VERBOSE_OUTPUT=false cargo run --release -q -- --solve

bench:
	@python3 tests.py

profile:
	@valgrind --tool=callgrind target/release/five-in-a-rust --perft 8
# kcachegrind callgrind.out.18203