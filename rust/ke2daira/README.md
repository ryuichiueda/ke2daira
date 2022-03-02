# ke2daira

## 何これ？

「苗字と名前の最初の文字を入れ替えてみました」のトリビュート・コマンドです。

* http://sledge-hammer-web.my.coocan.jp/names.htm

## 使い方

```
$ echo あとう かい | ke2daira
かとう あい
$ echo 阿寒 湖畔 温泉 | ke2daira -m
コカン アハン オンセン
$ echo デーモン 小暮閣下 | ke2daira -m
コーモン デグレカッカ
$ echo チェ ゲバラ | ke2daira 1.2 2.1
ゲ チェバラ
$ echo ゲバラ 焼肉の タレ | ke2daira -m 1.1 3.1
タバラ ヤキニクノ ゲレ
```

## インストール方法

```
$ sudo apt-get install -y mecab libmecab-dev mecab-ipadic-utf8 
$ cargo build --release
$ sudo cp target/release/ke2daira <どこかパスの通ったディレクトリ>
```
