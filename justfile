default: fmt test lint

version := "0.1.3"
tempdir := `mktemp -d`

fmt:
  cargo fmt --all --check
  nix fmt flake.nix

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

[private]
nix-hash target:
  @echo {{target}}
  @nix-prefetch-url --unpack https://github.com/akiomik/mado/releases/download/v{{version}}/{{target}}

hash-linux-arm64: (hash "mado-Linux-gnu-arm64.tar.gz.sha256")
hash-linux-amd64: (hash "mado-Linux-gnu-x86_64.tar.gz.sha256")
hash-macos-arm64: (hash "mado-macOS-arm64.tar.gz.sha256")
hash-macos-amd64: (hash "mado-macOS-x86_64.tar.gz.sha256")
hash-windows-amd64: (hash "mado-Windows-msvc-x86_64.zip.sha256")
list-hash: hash-linux-arm64 hash-linux-amd64 hash-macos-arm64 hash-macos-amd64 hash-windows-amd64

nix-hash-linux-arm64: (nix-hash "mado-Linux-gnu-arm64.tar.gz")
nix-hash-linux-amd64: (nix-hash "mado-Linux-gnu-x86_64.tar.gz")
nix-hash-macos-arm64: (nix-hash "mado-macOS-arm64.tar.gz")
nix-hash-macos-amd64: (nix-hash "mado-macOS-x86_64.tar.gz")
nix-hash-windows-amd64: (nix-hash "mado-Windows-msvc-x86_64.zip")
list-nix-hash: nix-hash-linux-arm64 nix-hash-linux-amd64 nix-hash-macos-arm64 nix-hash-macos-amd64 nix-hash-windows-amd64
