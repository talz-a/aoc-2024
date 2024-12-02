package main

import (
	"fmt"
	"os"

	"github.com/talz-a/aoc-2024/days"
)

func main() {
    if len(os.Args) < 2 {
        fmt.Println("USSAGE: go run main.go <day>")
        return
    }
    day := os.Args[1]
    switch day {
    case "day01": 
        days.Day01()
    default: 
        fmt.Println("ERROR: day not implemented")
    }
}
