package main

import (
	_ "embed"
	"flag"
	"fmt"
	"strings"

	"github.com/danifdz/advent-of-code/cast"
	"github.com/danifdz/advent-of-code/math"
	"github.com/danifdz/advent-of-code/util"
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

func validate(report []int) bool {
  dec := false
  for idx := range report {
    if idx == 0 {
      dec = report[idx] > report[idx+1]
    } else {
      diff := math.Abs(report[idx-1] - report[idx])
      decStep := report[idx-1] > report[idx]
      if (decStep && !dec) || (!decStep && dec) || diff < 1 || diff > 3 {
        return false
      }
    }
  }

  return true
}

func part1(input string) int {
	parsed := parseInput(input)
	ans := len(parsed)

	for _, report := range parsed {
    if validate(report) {
      ans -= 1
    }
	}

	return ans
}

func part2(input string) int {
	parsed := parseInput(input)
	ans := len(parsed)

	for _, report := range parsed {
    valid := false
    if validate(report) {
      valid = true
      util.PrintIntSlice(report)
      fmt.Println("- Valid")
      continue
    }

    for idx := range report {
      newReport := make([]int, 0)
      for j, n := range report {
        if j != idx {
          newReport = append(newReport, n)
        }
      }
      if validate(newReport) {
        valid = true
        util.PrintIntSlice(newReport)
        fmt.Println("- Valid")
        break
      }
    }

    if !valid {
      ans -= 1
      util.PrintIntSlice(report)
      fmt.Println("- Ivnalid")
    }
	}

	return ans
}

func parseInput(input string) (ans [][]int) {
	for _, line := range strings.Split(input, "\n") {
		report := make([]int, 0)
		for _, num := range strings.Fields(line) {
			report = append(report, cast.ToInt(num))
		}
		ans = append(ans, report)
	}
	return ans
}
