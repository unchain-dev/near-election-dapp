## ✅NEAR-Election-dApp(prototype)

本レポジトリは NEAR-Election-dApp の完成版を示したものになります。

以下の手順を実行することで NEAR-Election-dApp の挙動を確認できます。

## レポジトリのクローン

[NEAR-Election-dApp のリポジトリ](https://github.com/unchain-tech/NEAR-Election-dApp)から NEAR-Election-dApp をクローンします。

### コントラクトとフロントの準備

1. コントラクト開発の環境構築を行う

[NEAR-Election-dApp の教材](https://app.unchain.tech/learn/NEAR-Election-dApp/ja/0/2/)のうち section0-lessen2 の環境構築に従ってコントラクトが動くような環境を作りましょう。

2. ウォレットの作成とアドレスの取得

[NEAR-Election-dApp の教材](https://app.unchain.tech/learn/NEAR-Election-dApp/ja/1/4/)のうち section0-lessen2 の`mintしてみよう`以下の部分に記載してあるウォレットの作成を行いましょう。また、`exportコマンド`を用いて作成したウォレットアドレスをターミナルで使用できるようにしましょう。

また`packages/client/`下に neardev ディレクトリを作成し、その直下に`dev-account.env`というファイルを作成しましょう。
その中身は下のようにし、`YOUR_WALLET_ADDRESS`には先ほど作成したウォレットアドレスを入れましょう。

```
CONTRACT_NAME=YOUR_WALLET_ADDRESS
```

3. フロントエンドを起動し、動作確認

下のコマンドを実行することでフロントエンドを起動して動作確認をしましょう。

```
yarn client dev
```

動作確認は[NEAR-Election-dApp の教材](https://app.unchain.tech/learn/NEAR-Election-dApp/ja/3/1/)のうち section3-lessen1 を参考に行いましょう。
