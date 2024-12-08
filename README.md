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
| MD003 | :x:                | [mdast](https://github.com/syntax-tree/mdast) does not support heading styles.       |
| MD004 | :x:                | [mdast](https://github.com/syntax-tree/mdast) does not support list styles.          |
| MD005 | :warning:          | If the list item has no text, consistency cannot be checked correctly.               |
| MD009 | :white_check_mark: |                                                                                      |
| MD010 | :white_check_mark: |                                                                                      |
| MD012 | :white_check_mark: |                                                                                      |
| MD013 | :white_check_mark: |                                                                                      |
| MD022 | :white_check_mark: |                                                                                      |
