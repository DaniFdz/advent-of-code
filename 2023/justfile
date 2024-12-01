run day part:
	cargo run -r -p {{day}} --bin {{part}}

bench-all:
	cargo bench -q | tee benchmarks/benchmarks.txt

bench day:
	cargo bench --bench {{day}} part1 2>/dev/null | tee benchmarks/{{day}}.bench.txt
	cargo bench --bench {{day}} part2 2>/dev/null | tee -a benchmarks/{{day}}.bench.txt

test day:
	cargo test -p {{day}}

test-all:
	cargo test

create name:
	cargo generate --path ./template --name {{name}}
