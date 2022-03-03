package main

import "testing"

func checkSimpleReplacement(input string, output string, t *testing.T){
  result := ke2dairanization(input, 0, 1, 1, 1)
  if result != output {
    t.Fatalf("failed test " + input + "\n" + result + " != " + output)
  }
}

func checkMultiReplacement(input string, output string, f1 int, f1num int, f2 int, f2num int, t *testing.T){
  result := ke2dairanization(input, f1, f1num, f2, f2num)
  if result != output {
    t.Fatalf("failed test " + input + "\n" + result + " != " + output)
  }
}

func TestWords(t *testing.T) {
  checkSimpleReplacement("松平 健", "ケツダイラ マン", t)
  checkSimpleReplacement("阿寒 湖畔 温泉", "コカン アハン オンセン", t)
  checkSimpleReplacement("阿寒 湖畔 温泉 入浴", "コカン アハン オンセン ニュウヨク", t)
  checkMultiReplacement("チェ ゲバラ", "ゲ チェバラ", 0, 2, 1, 1, t)
  checkMultiReplacement("ゲバラ 焼肉の タレ", "タバラ ヤキニクノ ゲレ", 0, 1, 2, 1, t)
}
