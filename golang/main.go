package main

import (
	"fmt"
	"str/utils"
)

func main() {
	fmt.Printf("result convert from \"42\": %d\n", utils.ConvertToNumber("42"))
	fmt.Printf("result convert from \"abc\": %d\n", utils.ConvertToNumber("abc"))
	fmt.Printf("result compare string: %t\n", utils.CompareString("same string", "same string"))
}
