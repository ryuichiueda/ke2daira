package main

import "testing"

func TestExampleTwoWords(t *testing.T) {
  result := ke2dairanization("松平 健")
  if result != "ケツダイラ マン" {
      t.Fatalf("failed test " + "松平 健")
  }
}

func TestExampleThreeWords(t *testing.T) {
  result := ke2dairanization("阿寒 湖畔 温泉")
  if result != "コカン アハン オンセン" {
      t.Fatalf("failed test " + "阿寒 湖畔 温泉")
  }
}


func TestExampleMoreWords(t *testing.T) {
  result := ke2dairanization("阿寒 湖畔 温泉 入浴")
  if result != "コカン アハン オンセン ニュウヨク" {
      t.Fatalf("failed test " + "阿寒 湖畔 温泉 入浴")
  }
}

