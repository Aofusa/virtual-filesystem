Virtual Filesystem Wasm
=====


Virtual FileSystem の wasm 版  


ビルドやサーバの起動  
-----

以下のコマンドでサーバを起動し [localhost:8080](http://localhost:8080) にアクセスすると動作確認ができる  

```sh
# Cargo.toml のあるディレクトリで実行
wasm-pack test --firefox --headless
wasm-pack build

# www 配下に移動して実行
npm run start
```


初期構築  
-----

https://rustwasm.github.io/docs/book/game-of-life/hello-world.html

```sh
npm install
# もしエラーが出たら npm i g node-gyp をして gyp を先に入れておく
```

