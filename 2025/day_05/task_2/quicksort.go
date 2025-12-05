package main

func sortRanges(ranges []Range) {
	if len(ranges) < 2 {
		return
	}
	sortRangesHelper(ranges, 0, len(ranges)-1)
}

func sortRangesHelper(ranges []Range, low, high int) {
	if low < high {
		pivotIndex := partition(ranges, low, high)
		sortRangesHelper(ranges, low, pivotIndex-1)
		sortRangesHelper(ranges, pivotIndex+1, high)
	}
}

func partition(ranges []Range, low, high int) int {
	pivotIdx := low + (high-low)/2
	ranges[pivotIdx], ranges[high] = ranges[high], ranges[pivotIdx]

	pivot := ranges[high].first
	i := low - 1

	for j := low; j < high; j++ {
		if ranges[j].first < pivot {
			i++
			ranges[i], ranges[j] = ranges[j], ranges[i]
		}
	}
	ranges[i+1], ranges[high] = ranges[high], ranges[i+1]
	return i + 1
}
