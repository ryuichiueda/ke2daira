// license and copyright => see help()
package main

import (
  "os"
  "strconv"
  "bufio"
  "fmt"
  "github.com/shogo82148/go-mecab"
  "strings"
)

const VERSION = "0.2.2"

func help() {
  fmt.Fprintf(os.Stderr, "KETSUDAIRA COMMAND %s\n", VERSION)
  fmt.Fprintln(os.Stderr, "Copyright (C) 2019 Ryuichi Ueda.");
  fmt.Println()
  fmt.Fprintln(os.Stderr, "usage: ke2daira [f1.n] [f2.m]");
  fmt.Println()
  fmt.Fprintln(os.Stderr, "Released under the MIT license")
  fmt.Fprintln(os.Stderr, "https://github.com/ryuichiueda/ke2daira")
}

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

func readline () string {
  stdin := bufio.NewScanner(os.Stdin)
  if stdin.Scan() {
    return stdin.Text()
  }else{
    return ""
  }
}

func toYomi(s []string) [][]rune {
  var ans [][]rune;
  for i := range s {
    ans = append(ans, parse(s[i]) )
  }
  return ans
}

func toString(s [][]rune) string {
  var tmp []string;
  for i := range s {
    tmp = append(tmp, string(s[i]) )
  }
  return strings.Join(tmp, " ")
}

func ke2dairanization(line string, f1 int, f1num int, f2 int, f2num int) string {
  slice := toYomi( strings.Split(line, " ") )
  if len(slice) < 2 {
	  os.Exit(1)
  }

  f1_head := string(slice[f1][:f1num])
  f1_tail := string(slice[f1][f1num:])

  f2_head := string(slice[f2][:f2num])
  f2_tail := string(slice[f2][f2num:])

  slice[f1] = []rune(f2_head + f1_tail)
  slice[f2] = []rune(f1_head + f2_tail)

  return toString(slice)
}

func paramToNums (p string) (int, int) {
  f1 := strings.Split(p, ".")
  f1pos, _ := strconv.Atoi(f1[0])

  if len(f1) > 1 {
    f1num, _ := strconv.Atoi(f1[1])
    return f1pos-1, f1num
  } else {
    return f1pos-1, 1
  }
}

func main() {
  switch len(os.Args) {
  case 1:
    line := readline()
    result := ke2dairanization(line, 0, 1, 1, 1)
    fmt.Println(result)
  case 3:
    f1pos, f1num := paramToNums(os.Args[1])
    f2pos, f2num := paramToNums(os.Args[2])

    line := readline()
    result := ke2dairanization(line, f1pos, f1num, f2pos, f2num)
    fmt.Println(result)
  default:
    help()
    os.Exit(0)
  }
}
