CARGO=cargo

dev: Makefile
	$(CARGO) run -- debug --owner smallkirby --cache-dir ./vis-cache

.PHONY: dev
