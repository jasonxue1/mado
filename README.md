# downlint

A fast Markdown linter, written in Rust.

## Usage

```bash
downlint check .
downlint check path/to/*.md
```

## Rules

| Rule  | Support            | Note                                                                                 |
|-------|--------------------|--------------------------------------------------------------------------------------|
| MD001 | :white_check_mark: |                                                                                      |
| MD002 | :white_check_mark: |                                                                                      |
| MD003 | :waring:           | `:atx_closed` style does not support.                                                |
| MD004 | :waring:           | `:sublist` style does not support.                                                   |
| MD005 | :white_check_mark: |                                                                                      |
| MD009 | :white_check_mark: |                                                                                      |
| MD010 | :white_check_mark: |                                                                                      |
| MD012 | :white_check_mark: |                                                                                      |
| MD013 | :white_check_mark: |                                                                                      |
| MD022 | :white_check_mark: |                                                                                      |
