.PHONY: build test

build:
	(cd lib/skyapi && cargo build)

test:
	(cd lib/skyapi && cargo test)
