package main

import (
	"fmt"
	"strconv"
)

func isPalindrome(x int) bool {
	xString := strconv.Itoa(x)
	for i := 0; i < len(xString); i++ {
		if xString[len(xString) - i - 1] != xString[i] {
			return false
		}
	}

	return true
}

func main() {
	for _, x := range []int{1, 2, 1234, 121, 14541} {
		res := isPalindrome(x)
		fmt.Println(res)
	}
}
