package main

import (
	"fmt"
)

// https://leetcode.com/problems/string-to-integer-atoi/

type Bignum struct {
	number string
	negative bool
	value int32
}

func isNumber(x byte) bool {
	// first 127 of unicode tables equal to ascii table
	// stating point of numbers is 48 - '0'
	// finishing point of numbers is 57 - '9'
	ascii := int32(x)
	return ascii-48 >= 0 && ascii-48 < 10
}

func isSpace(x byte) bool {
	// ascii code of space is 32
	return int32(x) == 32
}

func isSign(x byte) bool {
	return int32(x) == 45 || int32(x) == 43
}

func isNegative(x byte) bool {
	// ascii code of '-' is 45
	return int32(x) == 45
}

func isPunctuation(x byte) bool {
	// ascii code of '.' is 46
	return int32(x) == 46
}

func invert(x int32) int32 {
	return -x
}

func byteToInt32(x byte) int32 {
	ascii := int32(x)
	return int32(ascii - 48)
}

func atoi(str string) int32 {
	var res int32
	//
	//bignum := Bignum{
	//	number:   str,
	//}
	//
	//bignum.number = strings.TrimSpace(bignum.number)

	// skip spaces
	var i int
	for i < len(str) {
		if isNumber(str[i]) || isSign(str[i]) {
			break
		}
		if !isSpace(str[i]) {
			return 0
		}

		i++
	}

	if i >= len(str) {
		return 0
	}

	var negative bool
	if isSign(str[i]) {
		if isNegative(str[i]) {
			negative = true
		}
		i++
	}

	fmt.Println(negative)

	// parse int number until non-number char
	var buf []byte
	for j := i; j < len(str); j++ {
		if !isNumber(str[j]) || isPunctuation(str[j]) {
			break
		}

		buf = append(buf, str[j])
		i++
	}

	if len(buf) == 0 {
		return 0
	}
	return res
}

func main() {
	fmt.Println(atoi("           2341341324123412341234123413241234123413241234123"))
}
