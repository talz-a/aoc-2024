package days

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func parseInput(filename string) ([]int, []int, error) {
    var left, right []int

    file, err := os.Open(filename)
    if err != nil {
        return nil, nil, fmt.Errorf("failed to open file %s: %w", filename, err)
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        fields := strings.Fields(scanner.Text())

        if len(fields) < 2 {
            continue
        }

        leftVal, err1 := strconv.Atoi(fields[0])
        rightVal, err2 := strconv.Atoi(fields[1])
        if err1 != nil || err2 != nil {
            return nil, nil, fmt.Errorf("failed to parse integers from line: %s", scanner.Text())
        }

        left = append(left, leftVal)
        right = append(right, rightVal)
    }

    return left, right, nil
}

func PartA(leftNums, rightNums []int) {
    if len(leftNums) != len(rightNums) {
        panic("left and right slices should have equal length")
    }

    slices.Sort(leftNums)
    slices.Sort(rightNums)

    totalDifference := 0
    for i := range leftNums {
        totalDifference += abs(leftNums[i] - rightNums[i])
    }

    fmt.Println("PART-A ANSWER:", totalDifference)
}

func PartB(leftNums, rightNums []int) {
    frequency := make(map[int]int)
    for _, num := range rightNums {
        frequency[num]++
    }

    total := 0
    for _, num := range leftNums {
        total += num * frequency[num]
    }

    fmt.Println("PART-B ANSWER:", total)
}

func abs(a int) int {
    if a < 0 {
        return -a
    }
    return a
}

func Day01() {
    leftNums, rightNums, err := parseInput("inputs/day1.txt")
    if err != nil {
        fmt.Println("ERROR:", err)
        return
    }
    PartA(leftNums, rightNums)
    PartB(leftNums, rightNums)
}
