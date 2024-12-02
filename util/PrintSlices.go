package util

import (
  "fmt"
)

func PrintIntSlice(s []int) {
  for _, n := range s {
    fmt.Printf("%d ", n)
  }
  fmt.Println()
}
