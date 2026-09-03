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

func main() {
	var total int64
	for index, size := uint64(0), sizeArgument(); index < size; index++ {
		value := int64(index%1_000) - 500
		total += value * value
	}
	fmt.Println(total)
}
