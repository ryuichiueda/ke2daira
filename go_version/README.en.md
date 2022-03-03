# ke2daira

[![Build Status](https://travis-ci.org/ryuichiueda/ke2daira.svg?branch=master)](https://travis-ci.org/ryuichiueda/ke2daira)

## What's this?

It's a tribute command to "KETSUDAIRA AWARD."

* http://sledge-hammer-web.my.coocan.jp/names.htm (in Japanese)

## How to use

### Ubuntu 18.04

```
$ sudo apt install golang
$ sudo apt install mecab mecab-ipadic-utf8 libmecab-dev
$ export CGO_LDFLAGS="`mecab-config --libs`"
$ export CGO_CFLAGS="-I`mecab-config --inc-dir`"
$ go get github.com/shogo82148/go-mecab
$ echo loopy potion | go run ke2daira.go
poopy lotion
```

### macOS (brew)

```
$ brew install golang
$ brew install mecab mecab-ipadic
$ export CGO_LDFLAGS="`mecab-config --libs`"
$ export CGO_CFLAGS="-I`mecab-config --inc-dir`"
$ go get github.com/shogo82148/go-mecab
$ echo loopy potion | go run ke2daira.go
poopy lotion
```
