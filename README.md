cwebp
---

画像変換ツール

## ビルド

```sh
cargo build --release
```


## 機能概要
- **ディレクトリ走査 & 画像検出**  
  指定した入力ディレクトリ以下から JPEG と PNG を再帰的に探し出します。

- **サイズ変更**  
  `--width`、`--height` オプションで出力画像の幅・高さを指定可能。未指定の場合は元サイズが使用されます。

- **フォーマット変換**  
  `--ext`（デフォルト: `avif`）で出力ファイル形式を変更します。対応フォーマットは `image` クレートがサポートするものです。

- **モノクロ化**  
  - `--force-monochrome` を付与すると全画像を強制的にモノクロ（Luma8）へ変換。  
  - デフォルトでは、入力画像がモノクロプロファイルの場合のみ自動でモノクロ化します。

- **積極的最適化** (`--aggressive-optimization`)  
  カラープロファイルの画像に対し色ピクセル比を計算し、カラー情報がほとんど無い場合はモノクロへ、そうでなければ RGB に変換してサイズ削減を試みます。

- **出力ディレクトリ自動作成**  
  必要なサブフォルダが存在しない場合は事前に `fs::create_dir_all` で作成します。

- **進捗表示**  
  `indicatif` のプログレスバーで処理数・経過時間・予測完了時刻をリアルタイムに表示します。

## 使用例
```bash
# 基本的な変換（幅800px, 高さ600px, 出力は avif）
cwebp -i ./photos -o ./converted -w 800 -h 600

# 強制モノクロ化で JPEG を PNG に変換
cwebp -i ./pics -o ./out --ext png --force-monochrome

# カラープロファイル判定と積極的最適化を有効にして変換
cwebp -i ./src_images -o ./dst_images --aggressive-optimization

# 低優先度で実行（Windows のみ）
cwebp -i ./photos -o ./converted --low-priority
```

## オプション一覧（`--help` で確認可）

| オプション | 説明 |
|------------|------|
| `-i, --input <PATH>` | 入力ディレクトリ（必須） |
| `-o, --output <PATH>` | 出力ディレクトリ（必須） |
| `-w, --width <U32>` | 変換後の幅（省略可、元サイズ使用） |
| `-h, --height <U32>` | 変換後の高さ（省略可、元サイズ使用） |
| `-e, --ext <STRING>` | 出力拡張子（デフォルト: `avif`） |
| `--force-monochrome` | 全画像をモノクロ化 |
| `--aggressive-optimization` | カラープロファイル判定と色ピクセル比に基づく最適化 |
| `--low-priority`            | Windows のみ有効。プロセス優先度を BELOW_NORMAL に設定し、システム負荷を軽減します。 |
| `--help` | ヘルプ情報表示 |
