package main

// Max-Heap for storing pairs by distance
type PairHeap []Pair

func (h PairHeap) Len() int           { return len(h) }
func (h PairHeap) Less(i, j int) bool { return sumSquares(h[i]) > sumSquares(h[j]) }
func (h PairHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *PairHeap) Push(x any) {
	*h = append(*h, x.(Pair))
}

func (h *PairHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func sumSquares(pair Pair) int {
	dx := int64(pair.a.x) - int64(pair.b.x)
	dy := int64(pair.a.y) - int64(pair.b.y)
	dz := int64(pair.a.z) - int64(pair.b.z)
	return int(dx*dx + dy*dy + dz*dz)
}

// Max-Heap for storing distances
type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(x any) {
	*h = append(*h, x.(int))
}

func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}
