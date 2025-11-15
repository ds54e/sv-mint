# Changelog

All notable changes to this project are documented here, following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) conventions and semantic versioning.

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

[1.1.3]: https://github.com/foo/sv-mint/releases/tag/v1.1.3
[1.1.2]: https://github.com/foo/sv-mint/releases/tag/v1.1.2
[1.1.1]: https://github.com/foo/sv-mint/releases/tag/v1.1.1
[1.1.0]: https://github.com/foo/sv-mint/releases/tag/v1.1.0
[1.0.0]: https://github.com/foo/sv-mint/releases/tag/v1.0.0
