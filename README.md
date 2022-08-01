## 🎫 **Near Election Contract**

## Web サイト

作成済みのアプリを見たい方は[こちら](https://tonny-near-election-dapp.netlify.app/)からご覧ください。

## **Quick Start**

---

1. [こちら](https://wallet.testnet.near.org/)で新しい wallet を作成してください

2. 作成した Wallet の ID を`YOUR_WALLET_ID`に代入して、下のコードをターミナルで走らせてください。

```
    export NFT_CONTRACT_ID="YOUR_WALLET_ID"
```

（例えば YOUR_CONTRACT_ID には dev_account.testnet などが入ることになります)

3. 下のコードをターミナルで走らせてログインしてください

```
    near login
```

4. 下のコードをターミナルで走らせてコンパイル、デプロイ、コントラクトの初期化をしてください

```
    near deploy --wasm-file target/wasm32-unknown-unknown/release/near_voting_contract.wasm --accountId $NFT_CONTRACT_ID && near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID
```

3. これでコントラクトの準備は完了です。では[こちら](https://github.com/honganji/near-election-dapp-frontend)からコードを clone した後、README.md の通りに準備をしてアプリを起動させましょう！
