nannouのスケッチ保管レポジトリ

# エクスポートについて
以下のようなコードで `./cap/プロジェクト名/フレーム数.png` でpngで書き出せる。
```Rust
fn view(_app: &App, _model: &Model, frame: Frame){
    let file_path = captured_frame_path(_app, &frame);
    _app.main_window().capture_frame(file_path);
}

fn captured_frame_path(_app: &App, frame: &Frame) -> std::path::PathBuf {
    _app.project_path()
        .expect("failed to locate `project_path`")
        .join("./cap/")
        .join(_app.exe_name().unwrap())
        .join(format!("{:04}", frame.nth()))
        .with_extension("png")
}
```

その後、

```sh
cd cap/プロジェクト名
ffmpeg -framerate 30 -i %04d.png -vcodec libx264 -pix_fmt yuv420p -r 60 out.mp4     
```
を行うことで動画を得られる。

ツイッターでの動画の規約は以下

>■動画の形式
>スマートフォンのアプリではMP4とMOVの動画形式をサポート。ブラウザではMP4（H264形式、AACオーディオ）をサポートしています。
>
>■動画のサイズ（容量）
>アップロードできるサイズは最大512MBです。
>
>■動画の時間
>アップロードできる動画の長さは2分20秒間以内です。
>
>■ブラウザでアップできる動画の解像度と縦横比
>最小解像度: 32 x 32
>最大解像度: 1920 x 1200、または1200 x 1900
>縦横比: 1:2.39～2.39:1の範囲
>最大フレームレート: 40fps
>最大ビットレート: 25Mbps

# ワークスペースをcargoで実行
親フォルダを開いた状態で、プロジェクトを指定して実行する場合に、以下のようなコマンドがあります。

```shell
cargo run --bin プロジェクト名 --release
```