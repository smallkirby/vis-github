CARGO=cargo

dev: Makefile
	$(CARGO) run -- --token "${GITHUB_API_TOKEN}" rate --owner smallkirby --cache-dir ./vis-cache

.PHONY: dev
