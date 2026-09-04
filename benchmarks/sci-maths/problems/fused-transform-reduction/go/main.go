package main

import (
	"fmt"
	"os"
	"strconv"
)

func sizeArgument() int {
	if len(os.Args) != 2 {
		fmt.Fprintln(os.Stderr, "expected exactly one positive integer size")
		os.Exit(2)
	}
	size, err := strconv.Atoi(os.Args[1])
	if err != nil || size <= 0 {
		fmt.Fprintln(os.Stderr, "size must be a positive integer")
		os.Exit(2)
	}
	return size
}

func transformed(index int) float64 {
	value := float64(index%1_000) / 100.0
	square := value * value
	return (square + 3.0*value - 7.0) / (1.0 + square)
}

func main() {
	var total float64
	for index, size := 0, sizeArgument(); index < size; index++ {
		total += transformed(index)
	}
	fmt.Println(total)
}
