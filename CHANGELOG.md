# Changelog

All notable changes to this project are documented here, following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) conventions and semantic versioning.

## [2.0.0] - 2025-11-21
### Removed
- Filelist support across the CLI (`-f/--filelist`, `+incdir`, `+define`, `-y`, `+libext`) along with related documentation and tests.

## [1.3.0] - 2025-11-15
### Added
- Introduced `docs/configuration.md`, a consolidated reference for every `sv-mint.toml` section, defaults, and script resolution rules.

### Changed
- Config loader now auto-populates defaults when `[defaults]`, `[plugin]`, `[logging]`, `[stages]`, `[svparser]`, or `[transport]` are omitted, so users can focus solely on `[[rule]]` entries.
- When rules omit `script`, the loader falls back to `./plugins/<rule>.<stage>.py` relative to the config file; pipeline stages without enabled rules are skipped and logged instead of invoking Python.
- README’s config sample was simplified to the minimal `[[rule]]` form and now links to the new configuration guide.

## [1.4.0] - 2025-11-16
### Changed
- Logging defaults now suppress stage, plugin, and parse events unless explicitly enabled.
- Config loader warns on unknown keys (including within `[[rule]]`) and enforces stricter validation for severity, transport limits, and plugin directories.
- Documentation clarifies timeout application per stage, transport value constraints, and rule/script validation; expanded config tests to cover partial overrides and error cases.

## [1.4.1] - 2025-11-17
### Changed
- Plugin resolution now works with user-specified `root` alone; `plugins/lib/rule_host.py` is resolved under `root` without needing bundled plugins. Added tests to cover this.

## [1.2.0] - 2025-11-15
### Changed
- Alphabetized the bundled `sv-mint.toml` rule list and renamed stale rule identifiers so script references remain accurate.
- Reworked plugin rule documentation: every rule now has its own file named after the script ID, the rule inventory index was removed, and README is aligned with the per-rule docs (including dropping redundant Japanese text).

## [1.1.3] - 2025-11-15
### Changed
- Pluginドキュメントを全面的に整形し、Rule表を箇条書きへ統一、Trigger/Remediationなどを見出し化、各ルールにGood/Bad例を追加して読みやすさを向上しました。
- READMEの構成を整理し、ユーザーがリリースバイナリを入手してすぐ利用できるよう基本手順を簡潔に説明する内容へ更新しました。

## [1.1.2] - 2025-11-14
### Fixed
- `naming_rules` plugin now avoids Python 3.8-only syntax, restoring compatibility with Python 3.6 environments such as RHEL8.

## [1.1.1] - 2025-11-14
### Added
- Plugin diagnostics now log the script path whenever a plugin errors, making it easier to identify failing plugins.

## [1.0.0] - 2025-11-14
- Initial stable release of sv-mint.

## [1.1.0] - 2025-11-14
### Added
- Filelist handling now supports `-f/--filelist`, `+incdir`, `+define`, `-y`, and `+libext` with nested includes, environment variable expansion, and quoting.
- `-y` directories combined with `+libext` trigger recursive auto-discovery of matching files (symlinks skipped, 50k safety limit).
- CLI gained `-f/--filelist` option to lint inputs described via svlint-style filelists.
- Tests now include filelist-driven smoke coverage, and README documents the supported syntax in detail.

[2.0.0]: https://github.com/foo/sv-mint/releases/tag/v2.0.0
[1.4.1]: https://github.com/foo/sv-mint/releases/tag/v1.4.1
[1.4.0]: https://github.com/foo/sv-mint/releases/tag/v1.4.0
[1.3.0]: https://github.com/foo/sv-mint/releases/tag/v1.3.0
[1.2.0]: https://github.com/foo/sv-mint/releases/tag/v1.2.0
[1.1.3]: https://github.com/foo/sv-mint/releases/tag/v1.1.3
[1.1.2]: https://github.com/foo/sv-mint/releases/tag/v1.1.2
[1.1.1]: https://github.com/foo/sv-mint/releases/tag/v1.1.1
[1.1.0]: https://github.com/foo/sv-mint/releases/tag/v1.1.0
[1.0.0]: https://github.com/foo/sv-mint/releases/tag/v1.0.0
