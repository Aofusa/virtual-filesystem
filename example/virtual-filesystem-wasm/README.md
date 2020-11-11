Virtual Filesystem Wasm
=====

Virtual FileSystem の wasm 版


初期構築  
https://rustwasm.github.io/docs/book/game-of-life/hello-world.html

ビルドやサーバの起動  
```sh
wasm-pack test --firefox --headless
wasm-pack build
npm install
# もしエラーが出たら npm i g node-gyp をして gyp を先に入れておく
npm run start
```

