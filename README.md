# RustでFizzBuzzゲームするWebAssembly

## ルール

1. 1~100までカウントする
2. 3の倍数の場合は文字列"Fizz"を追加
3. 5の倍数の場合は文字列"Buzz"を追加
4. 3の倍数かつ5の倍数の場合は文字列"Fizz Buzz"を追加
5. いずれでもない場合はカウントしている数を文字列として追加

## input

なし

## return

FizzBuzzゲームの結果を文字列で返す

## サンプルの動かし方

1. Cargo.tomlと同じ階層に移動
2. パッケージのビルド  
   `$ wasm-pack build --scope soichiro_dev`  

3. パッケージのウェブでの利用  
   `$ cd sample`  
   `$ npm install`  
   `$ npm run serve`  

4. これでウェブサーバーが起動します。`http://localhost:8080` を読み込んでください  
