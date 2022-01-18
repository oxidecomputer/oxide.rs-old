SHELL := bash

SPEC = $(CURDIR)/spec.json
SPEC_REPO = oxidecomputer/omicron
SPEC_REMOTE = https://raw.githubusercontent.com/oxidecomputer/omicron/main/openapi/nexus.json

VERSION = $(shell cat VERSION.txt)

generate: oxide
	cargo test tests
	cargo clippy

target/debug/generator: generator/src/*.rs generator/Cargo.toml
	cargo build --bin generator

update: update-specs

update-specs:
	$(RM) $(SPEC)
	make $(SPEC)

$(SPEC):
	curl -sSL $(SPEC_REMOTE) -o $@

oxide: target/debug/generator
	./target/debug/generator -i $(SPEC) -v $(VERSION) \
		-o oxide \
		-n oxide-api \
		-d "A fully generated & opinionated API client for the Oxide API." \
		--spec-link "https://github.com/$(SPEC_REPO)" $(EXTRA_ARGS)
	cargo fmt -p oxide-api

.PHONY: tag
tag: ## Create a new git tag to prepare to build a release.
	git tag -sa v$(VERSION) -m "v$(VERSION)"
	@echo "Run git push origin v$(VERSION) to push your new tag to GitHub and trigger a release."

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | sed 's/^[^:]*://g' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
