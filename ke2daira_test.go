package main

import "testing"

func TestExampleSuccess(t *testing.T) {
  result := ke2dairanization("松平 健")
  if result != "ケツダイラ マン" {
      t.Fatalf("failed test")
  }
}

