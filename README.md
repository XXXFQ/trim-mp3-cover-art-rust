# Trim MP3 Cover Art

MP3ファイルに埋め込まれているカバー画像（アルバムアート）を、  
中央基準で指定したアスペクト比（デフォルト：1:1）にトリミングし、再度埋め込むツールです。

PNG形式などの画像が埋め込まれている場合も、JPEGへ変換してから再埋め込みします。

Rust版では、処理速度と単体実行のしやすさを意識して実装しています。

---

## 使用方法

### 1. ディレクトリを指定する場合

```bash
trim-mp3-cover-art <mp3ディレクトリのパス>
````

例：

```bash
trim-mp3-cover-art ./music
```

---

### 2. ファイルを直接指定する場合

```bash
trim-mp3-cover-art song1.mp3 song2.mp3
```

例：

```bash
trim-mp3-cover-art song1.mp3 song2.mp3
```

---

### 3. サブディレクトリも含めて処理する場合

```bash
trim-mp3-cover-art ./music --recursive
```

短縮形：

```bash
trim-mp3-cover-art ./music -r
```

---

### 4. アスペクト比を指定する場合

デフォルトは `1.0`、つまり正方形です。

```bash
trim-mp3-cover-art song.mp3 --aspect-ratio 1.0
```

16:9にしたい場合：

```bash
trim-mp3-cover-art song.mp3 --aspect-ratio 1.777
```

短縮形：

```bash
trim-mp3-cover-art song.mp3 -a 1.777
```

---

### 5. ドラッグ＆ドロップ（Windows）

ビルド済みの `exe` に対して、

* MP3ファイル
* またはフォルダ

をドラッグ＆ドロップすることで、そのまま処理が実行されます。

---

## ビルド方法

### Windows

```powershell
cargo build --release
```

生成される実行ファイル：

```text
target\release\trim-mp3-cover-art.exe
```

---

### Linux / macOS

```bash
cargo build --release
```

生成される実行ファイル：

```text
target/release/trim-mp3-cover-art
```

---

## デバッグ実行

開発中に動作確認する場合は、以下のように実行できます。

```bash
cargo run -- ./music
```

ファイルを直接指定する場合：

```bash
cargo run -- song.mp3
```

アスペクト比を指定する場合：

```bash
cargo run -- song.mp3 --aspect-ratio 1.777
```

---

## ログ出力

ログファイルの出力は、**デバッグビルド時のみ**行います。

通常の開発実行：

```bash
cargo run -- song.mp3
```

ログ出力先：

```text
logs/debug.log
```

リリースビルドでは、ログファイルは作成されません。

```bash
cargo build --release
```

---

## Windows用アイコン

Windows版の実行ファイルにアイコンを埋め込む場合は、以下の場所に `.ico` ファイルを配置してください。

```text
assets/trim_mp3_icon.ico
```

その後、通常どおりリリースビルドします。

```powershell
cargo build --release
```

---

## 出力

処理結果は **元のMP3ファイルに直接反映** されます。

別ファイルとして出力するのではなく、指定したMP3ファイルの埋め込みカバー画像を直接更新します。

---

## 注意

* 処理は元ファイルを上書きします
* 必要に応じて事前にバックアップを取得してください
* 一部のMP3ではカバーアートが存在しない場合があります
* 対応していない画像形式や壊れたID3タグを含むMP3では処理に失敗する場合があります
* 初回実行時は、コピーしたMP3ファイルで動作確認することを推奨します

---

## 技術仕様

* Rust
* ID3タグ操作
* 画像処理（アスペクト比トリミング + JPEG変換）
* Windows `.exe` アイコン埋め込み
* デバッグビルド時のみログ出力

主な使用クレート：

* `clap`
* `id3`
* `image`
* `walkdir`
* `anyhow`
* `tracing`
* `tracing-subscriber`
* `winresource`

---

## プロジェクト構成

```text
trim-mp3-cover-art/
├── Cargo.toml
├── build.rs
├── assets/
│   └── trim_mp3_icon.ico
└── src/
    ├── main.rs
    ├── cli.rs
    ├── file_finder.rs
    ├── image_crop.rs
    ├── logger.rs
    └── mp3_processor.rs
```

---

## ライセンス

© 2025 ARM

````

PowerShellでそのまま作るなら、プロジェクト直下でこれです。

```powershell
@'
# Trim MP3 Cover Art

MP3ファイルに埋め込まれているカバー画像（アルバムアート）を、  
中央基準で指定したアスペクト比（デフォルト：1:1）にトリミングし、再度埋め込むツールです。

PNG形式などの画像が埋め込まれている場合も、JPEGへ変換してから再埋め込みします。

Rust版では、処理速度と単体実行のしやすさを意識して実装しています。

---

## 使用方法

### 1. ディレクトリを指定する場合

```bash
trim-mp3-cover-art <mp3ディレクトリのパス>
````

例：

```bash
trim-mp3-cover-art ./music
```

---

### 2. ファイルを直接指定する場合

```bash
trim-mp3-cover-art song1.mp3 song2.mp3
```

---

### 3. サブディレクトリも含めて処理する場合

```bash
trim-mp3-cover-art ./music --recursive
```

短縮形：

```bash
trim-mp3-cover-art ./music -r
```

---

### 4. アスペクト比を指定する場合

デフォルトは `1.0`、つまり正方形です。

```bash
trim-mp3-cover-art song.mp3 --aspect-ratio 1.0
```

16:9にしたい場合：

```bash
trim-mp3-cover-art song.mp3 --aspect-ratio 1.777
```

短縮形：

```bash
trim-mp3-cover-art song.mp3 -a 1.777
```

---

### 5. ドラッグ＆ドロップ（Windows）

ビルド済みの `exe` に対して、

* MP3ファイル
* またはフォルダ

をドラッグ＆ドロップすることで、そのまま処理が実行されます。

---

## ビルド方法

### Windows

```powershell
cargo build --release
```

生成される実行ファイル：

```text
target\release\trim-mp3-cover-art.exe
```

---

### Linux / macOS

```bash
cargo build --release
```

生成される実行ファイル：

```text
target/release/trim-mp3-cover-art
```

---

## デバッグ実行

```bash
cargo run -- ./music
```

ファイルを直接指定する場合：

```bash
cargo run -- song.mp3
```

アスペクト比を指定する場合：

```bash
cargo run -- song.mp3 --aspect-ratio 1.777
```

---

## ログ出力

ログファイルの出力は、**デバッグビルド時のみ**行います。

```bash
cargo run -- song.mp3
```

ログ出力先：

```text
logs/debug.log
```

リリースビルドでは、ログファイルは作成されません。

```bash
cargo build --release
```

---

## Windows用アイコン

Windows版の実行ファイルにアイコンを埋め込む場合は、以下の場所に `.ico` ファイルを配置してください。

```text
assets/trim_mp3_icon.ico
```

その後、通常どおりリリースビルドします。

```powershell
cargo build --release
```

---

## 出力

処理結果は **元のMP3ファイルに直接反映** されます。

別ファイルとして出力するのではなく、指定したMP3ファイルの埋め込みカバー画像を直接更新します。

---

## 注意

* 処理は元ファイルを上書きします
* 必要に応じて事前にバックアップを取得してください
* 一部のMP3ではカバーアートが存在しない場合があります
* 対応していない画像形式や壊れたID3タグを含むMP3では処理に失敗する場合があります
* 初回実行時は、コピーしたMP3ファイルで動作確認することを推奨します

---

## 技術仕様

* Rust
* ID3タグ操作
* 画像処理（アスペクト比トリミング + JPEG変換）
* Windows `.exe` アイコン埋め込み
* デバッグビルド時のみログ出力

主な使用クレート：

* `clap`
* `id3`
* `image`
* `walkdir`
* `anyhow`
* `tracing`
* `tracing-subscriber`
* `winresource`

---

## プロジェクト構成

```text
trim-mp3-cover-art/
├── Cargo.toml
├── build.rs
├── assets/
│   └── trim_mp3_icon.ico
└── src/
    ├── main.rs
    ├── cli.rs
    ├── file_finder.rs
    ├── image_crop.rs
    ├── logger.rs
    └── mp3_processor.rs
```

---

## ライセンス

© 2026 ARM
