package main

import (
	"fmt"
	"os"
	"strconv"
)

func sizeArgument() uint64 {
	if len(os.Args) != 2 {
		fmt.Fprintln(os.Stderr, "expected exactly one positive integer size")
		os.Exit(2)
	}
	size, err := strconv.ParseUint(os.Args[1], 10, 64)
	if err != nil || size == 0 {
		fmt.Fprintln(os.Stderr, "size must be a positive integer")
		os.Exit(2)
	}
	return size
}

func stoppingSteps(start uint64) uint64 {
	var steps uint64
	for value := start; value != 1; steps++ {
		if value%2 == 0 {
			value /= 2
		} else {
			value = 3*value + 1
		}
	}
	return steps
}

func main() {
	var total uint64
	for start, size := uint64(1), sizeArgument(); start <= size; start++ {
		total += stoppingSteps(start)
	}
	fmt.Println(total)
}
