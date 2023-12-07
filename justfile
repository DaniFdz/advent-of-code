run day part:
	cargo run -r -p {{day}} --bin {{part}}

bench-all:
	cargo bench -q | tee benchmaks.txt

bench day part:
	cargo bench --bench {{day}} {{part}} | tee -a {{day}}/{{day}}.bench.txt

test day:
	cargo test -p {{day}}

test-all:
	cargo test

create name:
	cargo generate --path ./template --name {{name}}
