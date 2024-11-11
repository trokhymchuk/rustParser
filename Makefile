all: build test run_example

build:
	cargo b

test:
	cargo test

clean:
	cargo clean

run_example:
	cargo r --  --file tests/iCals/1.ical

.PHONY: build, clean, run_example, all