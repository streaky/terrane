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

func generatedValue(index int) float64 {
	raw := float64(index%200 - 100)
	return 0.01*raw*raw + float64(index%7) - 3.0
}

func main() {
	values := make([]float64, sizeArgument())
	var sum float64
	for index := range values {
		value := generatedValue(index)
		values[index] = value
		sum += value
	}
	mean := sum / float64(len(values))
	var squaredDeviationSum float64
	for _, value := range values {
		deviation := value - mean
		squaredDeviationSum += deviation * deviation
	}
	variance := squaredDeviationSum / float64(len(values))
	outliers := 0
	for _, value := range values {
		deviation := value - mean
		if deviation*deviation > 2.5*variance {
			outliers++
		}
	}
	fmt.Println(outliers)
}
