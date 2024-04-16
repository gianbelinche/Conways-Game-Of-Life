csv_path ?= examples/glider.csv
milliseconds ?= 500

build:
	cargo build

run:
	cargo run "$(csv_path)" "$(milliseconds)"

glider:
	cargo run examples/glider.csv 500

space_ship:
	cargo run examples/space_ship.csv 500

glider_gun:
	cargo run examples/glider_gun.csv 100

test:
	cargo test

lint:
	cargo fmt
	cargo clippy -- -D clippy::nursery -D clippy::all -D clippy::complexity -A clippy::use_self
