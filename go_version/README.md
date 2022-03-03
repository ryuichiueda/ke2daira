# ke2daira

[![Build Status](https://travis-ci.org/ryuichiueda/ke2daira.svg?branch=master)](https://travis-ci.org/ryuichiueda/ke2daira)

## 何これ？

「苗字と名前の最初の文字を入れ替えてみました」のトリビュート・コマンドです。

* http://sledge-hammer-web.my.coocan.jp/names.htm

## 使い方

```
$ echo デーモン 小暮閣下 | go run ke2daira.go
コーモン デグレカッカ
$ echo 阿寒 湖畔 温泉 | go run ke2daira.go 
コカン アハン オンセン
$ echo チェ ゲバラ | go run ke2daira.go 1.2 2.1
ゲ チェバラ
$ echo ゲバラ 焼肉の タレ | go run ke2daira.go 1.1 3.1
タバラ ヤキニクノ ゲレ
```

## インストール方法

### Ubuntu 18.04

```
$ sudo apt install golang
$ sudo apt install mecab mecab-ipadic-utf8 libmecab-dev
$ export CGO_LDFLAGS="`mecab-config --libs`"
$ export CGO_CFLAGS="-I`mecab-config --inc-dir`"
$ go get github.com/shogo82148/go-mecab
```

### macOS (brew)

```
$ brew install golang
$ brew install mecab mecab-ipadic
$ export CGO_LDFLAGS="`mecab-config --libs`"
$ export CGO_CFLAGS="-I`mecab-config --inc-dir`"
$ go get github.com/shogo82148/go-mecab
$ echo ポール マッカートニー | go run ke2daira.go
マール ポッカートニー
```

## インストール

```
$ export GOPATH="<ホームディレクトリなど>/.go"
$ export GOBIN="$GOPATH/bin"
$ go install
$ PATH=$PATH:$GOPATH
$ echo loopy potion | ke2daira
poopy lotion
```
