package main

import (
	_ "embed"
	"flag"
	"fmt"
	"sort"
	"strings"

	"github.com/danifdz/advent-of-code/cast"
	"github.com/danifdz/advent-of-code/util"
	"github.com/danifdz/advent-of-code/math"
)

//go:embed input.txt
var input string

func init() {
	// do this in init (not main) so test file has same input
	input = strings.TrimRight(input, "\n")
	if len(input) == 0 {
		panic("empty input.txt file")
	}
}

func main() {
	var part int
	flag.IntVar(&part, "part", 1, "part 1 or 2")
	flag.Parse()
	fmt.Println("Running part", part)

	if part == 1 {
		ans := part1(input)
		util.CopyToClipboard(fmt.Sprintf("%v", ans))
		fmt.Println("Output:", ans)
	} else {
		ans := part2(input)
		util.CopyToClipboard(fmt.Sprintf("%v", ans))
		fmt.Println("Output:", ans)
	}
}

func part1(input string) (ans int) {
	list1, list2 := parseInput(input)
  sort.Slice(list1, func(i, j int) bool {
    return list1[i] < list1[j]
  })
  sort.Slice(list2, func(i, j int) bool {
    return list2[i] < list2[j]
  })

  for i, n1 := range list1 {
    ans += math.Abs(n1 - list2[i])
  }
  return
}

func part2(input string) (ans int) {
	list1, list2 := parseInput(input)
  l1 := Count(list1)
  l2 := Count(list2)

  for key, val := range l1 {
    el:= l2[key]
    ans += key * val * el
  }
  return ans
}

func Count(list []int) map[int]int {
  ans := make(map[int]int)
  for _, x := range list {
    val := ans[x]
    ans[x] = val + 1
  }
  return ans
}


func parseInput(input string) (list1 []int, list2  []int) {
	for _, line := range strings.Split(input, "\n") {
    var num1, num2 int
    for _, num := range strings.Split(line, " ") {
      if num != "" {
        if num1 == 0 {
          num1 = cast.ToInt(num)
        } else {
          num2 = cast.ToInt(num)
        }
      }
    }
    list1 = append(list1, num1)
    list2 = append(list2, num2)
	}
  return
}
