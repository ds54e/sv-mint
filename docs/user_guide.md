# sv-mint ユーザーガイド

## 想定読者
設計・検証エンジニアや EDA 管理者など、sv-mint を日常的に実行してレポートを解釈する人を対象にしています。基本的な SystemVerilog/CLI の知識を前提とし、プラグイン開発や内部実装の詳細は含みません。

## 1. セットアップ

### 1.1 必要環境
- Windows 10 以降 / macOS / Linux
- Rust stable toolchain（ビルド済みバイナリを配布する場合は不要）
- Python 3.x（`python3` もしくは `python` で起動できること）
- UTF-8 テキスト編集環境（BOM 付きでも可）

### 1.2 ビルド手順
```
rustup default stable
cargo build --release
```
ビルド後は `target/release/sv-mint`（Windows は `.exe`）をパスの通った場所に配置します。CI 等で高速化したい場合は `cargo build --release --locked` を推奨します。

### 1.3 Python ランタイム
`[plugin]` セクションの `cmd` / `args` に従って Python ホストを起動します。既定値では `python3 -u -B plugins/lib/rule_host.py` が呼び出されるため、virtualenv を利用する場合は `cmd` にフルパスを設定してください。

## 2. 実行方法

### 2.1 基本的な CLI
```
sv-mint --config ./sv-mint.toml path/to/a.sv path/to/b.sv
```
- `--config` を省略するとカレントディレクトリの `sv-mint.toml` を読み込みます。
- 入力は複数指定可。2 件以上の場合はワーカースレッドが自動的に並列実行します。

### 2.2 終了コード
| コード | 説明 |
| --- | --- |
| 0 | 違反無し |
| 2 | 違反を検出（warning / error） |
| 3 | 設定読込失敗、parse 失敗、プラグイン異常、タイムアウトなど |

### 2.3 代表的なオプション
| オプション | 説明 |
| --- | --- |
| `--config <path>` | TOML 設定ファイルを明示的に指定 |
| `<input ...>` | 解析対象の SystemVerilog ファイル群 |

## 3. `sv-mint.toml` 設定
主要セクションと用途を以下にまとめます。値は TOML 形式で記述してください。

### 3.1 `[defaults]`
| キー | 型 | 説明 |
| --- | --- | --- |
| `timeout_ms_per_file` | u64 | 1 ファイル当たりのプラグイン処理タイムアウト（ミリ秒） |

### 3.2 `[plugin]`
| キー | 型 | 説明 |
| --- | --- | --- |
| `cmd` | string | Python 実行コマンド（例: `python3`） |
| `args` | [string] | `rule_host.py` 起動時の追加引数 |

### 3.3 `[ruleset]`
| キー | 説明 |
| --- | --- |
| `scripts` | 実行したい Python ルールのパス配列（ロード順＝実行順） |
| `override` | ルール ID → `error|warning|info` のマップ。CLI で出力される Severity を上書き |

### 3.4 `[stages]`
| キー | 説明 |
| --- | --- |
| `enabled` | 実行するステージの列挙。`raw_text` / `pp_text` / `cst` / `ast` に対応 |

### 3.5 `[svparser]`
| キー | 説明 |
| --- | --- |
| `include_paths` | `sv-parser` に渡す `+incdir` 相当のパス配列 |
| `defines` | 事前定義する `` `define`` 記法（`NAME=VALUE` 形式） |
| `strip_comments` | コメントを前処理段階で除去するか |
| `ignore_include` | `include` を無視して単一ファイルで解析するか |
| `allow_incomplete` | パーサに不完全構文を許容させるか |

### 3.6 `[logging]`
| キー | 説明 |
| --- | --- |
| `level` | `error/warn/info/debug/trace` |
| `format` | `text` または `json` |
| `stderr_snippet_bytes` | プラグイン stderr を保持するバイト数（0 で無効） |
| `show_*_events` | `stage` / `plugin` / `parse` イベントの出力 ON/OFF |

### 3.7 設定テンプレート
`README.md` の [サンプル設定](../README.md#サンプル設定) をベースに、自組織の include path やルール構成を差し替えてください。

## 4. ログと診断の読み方

### 4.1 CLI 出力
`<path>:<line>:<col>: [severity] <rule_id>: <message>` の形式で 1 行 1 違反を表示します。`location.file` が指定されている場合は include 先のパスがそのまま表示されます。

### 4.2 `tracing` イベント
- `sv-mint::event`: ステージ開始/終了、プラグイン invoke/done、タイムアウト、stderr スニペットなどを info ログで出力。
- `sv-mint::stage`: ステージごとの統計（所要時間、違反件数、skip/fail 状態）。
`logging.format = "json"` にすると構造化ログとして収集できます。

### 4.3 サイズガードとタイムアウト
- リクエスト JSON が 12 MB を超えると warning、16 MB を超えると required stage はエラー、それ以外はスキップになります。
- プラグインがタイムアウトするとプロセスを kill し、`PluginTimeout` イベントと `sys.stage.timeout` 相当の診断を出力します。

## 5. FAQ（抜粋）

**Q. Windows で CRLF のままでも解析できますか？**  
A. `read_input` が LF へ正規化し、位置情報は元ファイルのバイト列を参照するため問題ありません。

**Q. ルールごとに重大度を変えたい。**  
A. `[ruleset.override]` に `"rule.id" = "warning"` のように記載すると、プラグイン実装を触らずに CLI 出力だけ書き換えられます。

**Q. include 先で発生した違反のファイル名が知りたい。**  
A. v2.7 以降は `Location.file` に実ファイルパスが入り、CLI でもそのまま表示されます。古いバージョンではソースのバイト列が必要でした。

より高度な情報（payload 仕様、プロトコル、アーキテクチャ）は `docs/plugin_author.md` や `docs/internal_spec.md` を参照してください。
