SHELL := bash

SPEC = $(CURDIR)/spec.json
SPEC_REPO = oxidecomputer/omicron
SPEC_REMOTE = https://raw.githubusercontent.com/oxidecomputer/omicron/main/openapi/nexus.json

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
	./target/debug/generator -i $(SPEC) -v 0.1.0-rc.6 \
		-o oxide \
		-n oxide-api \
		--proper-name Oxide \
		-d "A fully generated & opinionated API client for the Oxide API." \
		--spec-link "https://github.com/$(SPEC_REPO)" \
		--host "api.oxide.computer" $(EXTRA_ARGS)
	cargo fmt -p oxide-api
