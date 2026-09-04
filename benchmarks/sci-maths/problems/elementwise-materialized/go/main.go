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
	return value*value + 3.0*value - 7.0
}

func main() {
	values := make([]float64, sizeArgument())
	for index := range values {
		values[index] = transformed(index)
	}
	var total float64
	for _, value := range values {
		total += value
	}
	fmt.Println(total)
}
