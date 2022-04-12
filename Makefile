benchmark:
	@cargo run --release -q -- --rollout 1000000

perft:
	@cargo run --release -q -- --perft 4

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
