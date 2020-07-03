package main

import "log"

// https://leetcode.com/problems/palindromic-substrings/

import (
	"fmt"
	"log"
)

func isPalindrome(x string) bool {
	for i := 0; i < len(x); i++ {
		if x[len(x)-i-1] != x[i] {
			return false
		}
	}

	return true
}

// based on map
func palindromeMap(s string, left, right int, keeper map[string]int) {
	var windVane bool
	for left >= 0 && right < len(s) {
		if s[left] == s[right] {
			if isPalindrome(s[left : right+1]) {
				keeper[s[left:right+1]] += 1
			}
		}

		switch windVane {
		case false:
			right++
			break
		case true:
			left--
			break
		}

		windVane = !windVane
	}
}

func countSubstrings(s string) int {
	keeper := make(map[string]int)
	for i := 0; i < len(s); i++ {
		palindromeMap(s, i, i, keeper)
	}

	fmt.Println(keeper)

	var count int
	for _, v := range keeper {
		count += v
	}

	return count
}

func main() {
	for x, expected := range map[string]int{"abc": 3, "aaa": 6, "aba": 4, "abba": 6, "abbba": 9, "fdsklf": 6} {
		res := countSubstrings(x)
		if expected != res {
			log.Printf("-- %s -- expected %d, but given %d\n", x, expected, res)
		}
	}
}
