default: fmt test lint

version := "0.1.3"
tempdir := `mktemp -d`

fmt:
  cargo fmt --all --check

test:
  cargo test --all-features --workspace

lint:
  cargo clippy --all-targets --all-features --workspace -- -D warnings

cov:
  cargo llvm-cov --open

[linux]
flamegraph target="scripts/benchmarks/data/gitlab":
  cargo flamegraph --profile bench --open -- check {{target}}

[macos]
flamegraph target="scripts/benchmarks/data/gitlab":
  # See https://github.com/flamegraph-rs/flamegraph#dtrace-on-macos
  cargo flamegraph --root --profile bench --open -- check {{target}}

fuzz target="linter":
  cargo +nightly fuzz run {{target}}

[private]
hash target:
  @echo {{target}}
  @wget -q -P {{tempdir}} https://github.com/akiomik/mado/releases/download/v{{version}}/{{target}}
  @cut -d ' ' -f 1 {{tempdir}}/{{target}}

hash-linux-arm64: (hash "mado-Linux-gnu-arm64.tar.gz.sha256")
hash-linux-amd64: (hash "mado-Linux-gnu-x86_64.tar.gz.sha256")
hash-macos-arm64: (hash "mado-macOS-arm64.tar.gz.sha256")
hash-macos-amd64: (hash "mado-macOS-x86_64.tar.gz.sha256")
hash-windows-amd64: (hash "mado-Windows-msvc-x86_64.zip.sha256")
list-hash: hash-linux-arm64 hash-linux-amd64 hash-macos-arm64 hash-macos-amd64 hash-windows-amd64
