# Changelog

## 0.2.0 (2025-01-30)

### ⚠️ BREAKING CHANGES

* fix!: rename config keys in [lint.md030] by @akiomik in #86
* feat!: change style format for [lint.md035] by @akiomik in #91

### Features

* feat: add stdin support to check by @akiomik in #89
* feat: json schema support by @akiomik in #88

### Bug Fixes

* fix: check command with empty stdin by @akiomik in #96

### Other Changes

* chore: update packages to 0.1.5 by @akiomik in #85
* chore: add update-winget to justfile by @akiomik in #84
* chore: add breaking change to .github/release.yml by @akiomik in #87
* Taplo ci by @akiomik in #90
* build(deps): bump clap from 4.5.26 to 4.5.27 by @dependabot in #94
* build(deps): bump comrak from 0.33.0 to 0.35.0 by @dependabot in #95
* build(deps): bump rand from 0.8.5 to 0.9.0 by @dependabot in #93

## 0.1.5 (2025-01-22)

### Features

* Winget by @akiomik in #74
* feat: add --quiet flag by @hougesen in #78
* feat: add Serialize for Config by @akiomik in #81

### Bug Fixes

* fix: respect config with --quiet by @akiomik in #80

### Other Changes

* Run justfile --fmt by @akiomik in #68
* Update packages to v0.1.4 by @akiomik in #67
* Remove .cargo/config.toml by @akiomik in #69
* Use rust 1.84 by @akiomik in #70
* Nursery by @akiomik in #71
* Update README.md by @akiomik in #72
* Fix use_self by @akiomik in #73
* Add test for MarkdownLintVisitorFactory#build by @akiomik in #75
* Add test for ParallelLintRunner#run by @akiomik in #76
* ci: update .github/release.yml by @akiomik in #79

## 0.1.4 (2025-01-17)

* Minor improvements (#41, #42, #45, #46, #49)
* Bump colored from 2.2.0 to 3.0.0 (#43)
* Bump clap from 4.5.23 to 4.5.26 (#44)
* Add fuzz testing (#47)
* Update README.md (#48, #50)
* Add Homebrew support (#51, #52, #54, #55, #56, #57, #62)
* Add Scoop support (#53, #58)
* Add justfile (#59)
* Add Nix support (#60, #61)
* Add `.github/release.yml` (#63, #65)

## 0.1.3 (2025-01-13)

* Update project `mado.toml` (#13)
* Minor improvements (#19, #20, #23, #26, #29, #31, #32, #35, #39)
* Add tests (#21, #22, #33, #34, #36, #37, #38)
* Bump comrak from 0.32.0 to 0.33.0 (#24)
* Fix benchmark results (#25)
* Improve CI (#27, #30)
* Update README.md (#28)

## 0.1.2 (2025-01-05)

* Update `README.md` (#12, #17)
* Fix MD013 (#14)
* Fix `Cargo.toml` (#15)
* Add `Document#lines` (#16)

## 0.1.1 (2025-01-05)

* Add github action support (#7, #8)
* Add `code_blocks` and `tables` options to MD013 (#9)
* Fix global configuration loading (#10)

## 0.1.0 (2024-12-31)

* Initial release!
