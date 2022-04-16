perft:
	@cargo run --release -q -- --perft 11

test:
	@python3 -m pytest -s tests.py

solve:
	@/usr/bin/time -f "%M kBs of ram" cargo run --release -q -- --solve

# same as solve, but with output that is easier to parse
quiet:
	@VERBOSE_OUTPUT=false cargo run --release -q -- --solve

bench:
	@python3 tests.py

profile:
	@valgrind --tool=callgrind target/release/five-in-a-rust --perft 8
# kcachegrind callgrind.out.18203