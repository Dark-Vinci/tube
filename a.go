package main

import "fmt"

func mySqrt(x int) int {
	if x < 2 {
		return x
	}

	var result = x
	l, r := 0, x

	for {
		fmt.Println(l, r)
		h := (l + r) / 2

		if l * l < x && r * r > x && l + 1 == r {
		    return l
		}

		if h*h == x {
			result = h
			break
		} else if h*h < x {
			l = h
		} else {
			r = h
		}

	}

	return result
}

func main() {
	var result = mySqrt(8)
	fmt.Println(result)
	fmt.Println("Hello, 世界")
}