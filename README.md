# freee-accounting-sdk-rustリポジトリを使ったデモプログラム

このリポジトリのコードは、Raspberry Piでコンパイルします。

Windowsや、MacOS上では、コンパイルできないので注意してください。

## プログラムの内容
- Raspberry PiのGPIOに結線されたスイッチが押されたら、freee会計のPublic API経由で、経費申請を下書き作成します。
- スイッチは、GPIOの21番とGNDに接続し、スイッチ押下時に回路が短絡するようにします。
- Raspberry Pi内部のプルアップ回路を使用するので、回路上にプルアップ抵抗などを使う必要はありません。

このリポジトリと同じ階層に、[freee-accounting-sdk-rust](https://github.com/flipfrog/freee-accounting-sdk-rust)リポジトリの内容を配置します。

## 実行のための準備
- freeeのアプリストアで、新規にアプリを登録します。
- アプリ設定のコールバックURLに、Raspberry Piのホスト名を使い、`http://<host>:8080/auth_callback` を設定し、保存します。
- cloneした、freee-accounting-sdk-rustリポジトリの内容を変更します。
    - 下記に書かれているURLを、先に設定したコールバックURLに修正します。
    - https://github.com/flipfrog/freee-accounting-sdk-rust/blob/72e1f11fa3d5c19b92cf8aaf097152d8fc6fd416/examples/web/src/main.rs#L161 
    - 下記に書かれているホスト名を、`0.0.0.0` に修正します。
    - https://github.com/flipfrog/freee-accounting-sdk-rust/blob/72e1f11fa3d5c19b92cf8aaf097152d8fc6fd416/examples/web/src/main.rs#L192
- Raspberry Pi 上で、freee-accounting-sdk-rustのwebサンプルを実行して、アプリを認証します。認証した時点で、sqliteデータベースにfreee Public APIのアクセストークンが格納されます。
  認証時に、経費申請を作成したい事業所のIDをメモします。
- 環境変数 `RUST_API_EXAMPLE_COMPANY_ID` に、上記の経費申請を作成したいfreeeの事業所IDを設定します。

## コンパイルと実行方法

- `cargo run` を実行します。

## 経費申請の作成方法

- Raspberry Piにつないた、スイッチを押します。
