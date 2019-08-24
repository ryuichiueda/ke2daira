package main

import "testing"

func checkWord(input string, output string, t *testing.T){
  result := ke2dairanization(input)
  if result != output {
    t.Fatalf("failed test " + input + "\n" + result + " != " + output)
  }
}

func TestWords(t *testing.T) {
  checkWord("松平 健", "ケツダイラ マン", t)
  checkWord("阿寒 湖畔 温泉", "コカン アハン オンセン", t)
  checkWord("阿寒 湖畔 温泉 入浴", "コカン アハン オンセン ニュウヨク", t)
}
