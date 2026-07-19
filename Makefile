.PHONY: default help clean build test ci bench build_test fmt fmt-check clippy create_docs docs ayce tree tree-duplicates watch install-watch

# Default target
default: ayce

# Display help information
help:
	@echo "Available targets:"
	@echo "  make (default)       - Run ayce"
	@echo "  make build           - Build the project"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make test            - Run tests"
	@echo "  make ci              - Mirror GitHub Actions test job (RUSTFLAGS=-Dwarnings)"
	@echo "  make bench           - Run criterion benchmarks"
	@echo "  make build_test      - Clean, build, test, and doc tests"
	@echo "  make fmt             - Format code"
	@echo "  make fmt-check       - Check formatting without modifying (matches CI)"
	@echo "  make clippy          - Run clippy linter (matches CI strictness)"
	@echo "  make create_docs     - Build documentation"
	@echo "  make docs            - Build docs and open in browser"
	@echo "  make ayce            - Run fmt, build_test, clippy, and create_docs"
	@echo "  make help            - Display this help message"
	@echo ""
	@echo "Dependencies:"
	@echo "  make tree            - Show dependency tree"
	@echo "  make tree-duplicates - Show duplicate dependencies"
	@echo ""
	@echo "Tools and Workflow:"
	@echo "  make watch           - Run cargo-watch for check/test loop"
	@echo "  make install-watch   - Install cargo-watch"
	@echo ""

# Clean build artifacts
clean:
	cargo clean

# Build the project
build:
	cargo build

# Run tests
test:
	cargo test

# Mirror the GitHub Actions test job: warnings are hard errors.
ci:
	RUSTFLAGS="-Dwarnings" cargo test --all

# Run criterion benchmarks
bench:
	cargo bench

# Clean once, then build, run tests, and run doc tests
build_test: clean build test
	cargo test --doc

# Format code
fmt:
	cargo fmt --all

# Check formatting without modifying files (matches the CI fmt job)
fmt-check:
	cargo fmt --all -- --check

# Run clippy at the same strictness as the CI clippy job
clippy:
	cargo clippy -- -Dclippy::all -Dclippy::pedantic

# Show dependency tree
tree:
	cargo tree

# Show duplicate dependencies
tree-duplicates:
	cargo tree --duplicates

# Create documentation
create_docs:
	cargo doc --no-deps

# Open documentation in browser
docs: create_docs
	@DOC_PATH="./target/doc/rnglib/index.html"; \
	if command -v xdg-open >/dev/null 2>&1; then \
		xdg-open "$$DOC_PATH"; \
	elif command -v open >/dev/null 2>&1; then \
		open "$$DOC_PATH"; \
	else \
		echo "No supported opener found (tried xdg-open and open)."; \
		echo "Open $$DOC_PATH manually."; \
		exit 1; \
	fi

# All You Can Eat - Run all checks at CI strictness.
# Target-specific exports propagate to every prerequisite recipe (build,
# test, doc tests, clippy), so warnings become hard errors exactly like
# the GitHub Actions jobs. Standalone targets (e.g. `make test`) stay lenient.
ayce: export RUSTFLAGS := -Dwarnings
ayce: fmt build_test clippy create_docs

# Watch mode for development (requires cargo-watch)
watch:
	cargo watch -x check -x test

# Install cargo-watch
install-watch:
	cargo install cargo-watch
