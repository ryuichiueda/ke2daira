# ke2daira

## 何これ？

「苗字と名前の最初の文字を入れ替えてみました」のトリビュート・コマンドです。

* http://sledge-hammer-web.my.coocan.jp/names.htm

## 使い方

### Ubuntu 18.04

```
$ sudo apt install golang
$ sudo apt install mecab mecab-ipadic-utf8 libmecab-dev
$ export CGO_LDFLAGS="`mecab-config --libs`"
$ export CGO_CFLAGS="-I`mecab-config --inc-dir`"
$ go get github.com/shogo82148/go-mecab
$ echo デーモン 小暮閣下 | go run ke2daira.go
コーモン デグレカッカ
```
