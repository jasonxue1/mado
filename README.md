# downlint

A fast Markdown linter, written in Rust.
Compliant with [CommonMark](https://commonmark.org) and [GFM](https://github.github.com/gfm/).

## Usage

```bash
downlint check .
downlint check path/to/*.md
```

## Performance

36-44x faster than existing linters
(like [markdownlint](https://github.com/markdownlint/markdownlint)).

```mermaid
---
config:
    xyChart:
        height: 200
        titleFontSize: 14
        chartOrientation: horizontal
        xAxis:
            labelFontSize: 12
            titleFontSize: 14
        yAxis:
            labelFontSize: 12
            titleFontSize: 14
---
xychart-beta
    title "Linting the GitLab documents from scratch (Lower is faster)"
    x-axis ["downlint (rust)", "markdownlint-cli (node.js)", "markdownlint (ruby)", "markdownlint-cli2 (node.js)"]
    y-axis "Time (seconds)" 0 --> 10
    bar [0.177, 6.372, 6.464, 7.776]
```

This benchmark was run on a MacBook Pro (2021, M1 Max)
using [hyperfine](https://github.com/sharkdp/hyperfine),
and [the GitLab documentation](https://gitlab.com/gitlab-org/gitlab/-/tree/7d6a4025a0346f1f50d2825c85742e5a27b39a8b/doc)
is used (1,500 Markdown files).

## Rules

Several [markdownlint rules](https://github.com/markdownlint/markdownlint/blob/main/docs/RULES.md)
are currently supported.

* :white_check_mark: The rule is stable.
* :hammer: The rule is unstable.
* :warning: The rule has unsupported option(s).
* :x: The rule is unsupported.

| Rule  | Support            | Note                                         |
|-------|--------------------|----------------------------------------------|
| MD001 | :white_check_mark: |                                              |
| MD002 | :white_check_mark: |                                              |
| MD003 | :warning:          | `:atx_closed` style does not support.        |
| MD004 | :warning:          | `:sublist` style does not support.           |
| MD005 | :white_check_mark: |                                              |
| MD006 | :white_check_mark: |                                              |
| MD007 | :hammer:           |                                              |
| MD009 | :white_check_mark: |                                              |
| MD010 | :white_check_mark: |                                              |
| MD012 | :white_check_mark: |                                              |
| MD013 | :hammer:           |                                              |
| MD014 | :white_check_mark: |                                              |
| MD018 | :white_check_mark: |                                              |
| MD019 | :white_check_mark: |                                              |
| MD022 | :white_check_mark: |                                              |
| MD023 | :white_check_mark: |                                              |
| MD024 | :warning: :hammer: | `allow_different_nesting` does not support.  |
| MD025 | :white_check_mark: |                                              |
| MD026 | :white_check_mark: |                                              |
| MD027 | :hammer:           |                                              |
| MD028 | :white_check_mark: |                                              |
| MD029 | :white_check_mark: |                                              |
| MD030 | :white_check_mark: |                                              |
| MD031 | :white_check_mark: |                                              |
| MD032 | :white_check_mark: |                                              |
| MD033 | :hammer:           |                                              |
| MD034 | :white_check_mark: |                                              |
| MD035 | :x:                | comrak does not support line styles.         |
| MD036 | :hammer:           |                                              |
| MD037 | :hammer:           |                                              |
| MD038 | :white_check_mark: |                                              |
| MD039 | :white_check_mark: |                                              |
| MD040 | :white_check_mark: |                                              |
| MD041 | :white_check_mark: |                                              |
| MD046 | :white_check_mark: |                                              |
| MD047 | :white_check_mark: |                                              |

## Development

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy --all-targets --all-features --workspace -- -D warnings
```

### Benchmark

```bash
# Download markdown dataset
./scripts/benchmarks/setup.sh

# Run downlint, mdl and markdownlint-cli with hyperfine
./scripts/benchmarks/comparison.sh
```
