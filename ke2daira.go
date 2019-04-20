package main

import (
  "os"
  "bufio"
  "fmt"
  "github.com/shogo82148/go-mecab"
  "strings"
)

func parse(input string) []rune {
  tagger, err := mecab.New(map[string]string{"output-format-type": "yomi"})
  if err != nil {
      panic(err)
  }
  defer tagger.Destroy()

  result, err := tagger.Parse(input)
  if err != nil {
      panic(err)
  }
  return []rune(strings.Trim(result, "\n"))
}

func main() {
  stdin := bufio.NewScanner(os.Stdin)
  for stdin.Scan(){
    str := stdin.Text()
    slice := strings.Split(str, " ")
    var first_head, first_tail, second_head, second_tail string
    for n, s := range slice {
      yomi := parse(s)
      if n == 0 {
	first_head = string(yomi[:1])
	first_tail = string(yomi[1:])
      }
      if n == 1 {
	second_head = string(yomi[:1])
	second_tail = string(yomi[1:])
      }
    }
    fmt.Println(second_head + first_tail + " " + first_head + second_tail)
  }
}
