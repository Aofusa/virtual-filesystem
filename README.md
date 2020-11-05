Virtual Filesystem
=====

[![CircleCI](https://circleci.com/gh/Aofusa/virtual-filesystem.svg?style=svg)](https://circleci.com/gh/Aofusa/virtual-filesystem)


Rust で作るインメモリな擬似ファイルシステムみたいなもの  


実行
-----

起動するとシェルっぽいREPLが立ち上がります  
exit と打つか Ctrl + c で終了できる  

```sh
$ cargo run
start interactive shell. Enjoy! :/
to stop, press Ctrl + c or type exit
if you need help, type :?
$> 
mkdir dir
$> 
touch file
$> 
ls
dir	file
$> 
exit
```

```sh
cargo test
```

使えるコマンド一覧は以下の通り  

- ls  
  現在のディレクトリのファイル・フォルダ名一覧を表示  
- pwd  
  現在の位置を表示  
- cd  
  ディレクトリ移動  
- find  
  ファイル・ディレクトリがあるか確認  
- mkdir  
  ディレクトリ作成  
- touch  
  ファイル作成  
- read  
  ファイルの内容を読み取る  
- write  
  ファイルに書き込む  
- exit  
  シェルを終了する  
- :?  
  ヘルプを表示  


Dockerized
------

ビルド
```sh
docker build -t dev.local/virtual-filesystem .
```

実行
```sh
docker run -it --rm --name virtual-filesystem dev.local/virtual-filesystem
```

