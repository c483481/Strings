package main

import (
	"fmt"
	"str/utils"
)

func main() {
	fmt.Printf("result convert from \"42\": %d\n", utils.ConvertToNumber("42"))
	fmt.Printf("result convert from \"abc\": %d\n", utils.ConvertToNumber("abc"))
	fmt.Printf("result compare string: %t\n", utils.CompareString("same string", "same string"))
	fmt.Printf("result compare string: %t\n", utils.SafeCompareString("same string", "same string"))
	fmt.Printf("is email admin@gmail.com: %t\n", utils.IsEmail("admin@gmail.com"))
	fmt.Printf("is email asdf@adsf.adsf: %t\n", utils.IsEmail("asdf@adsf.adsf"))
	fmt.Printf("is email asdf@univ.ac.id: %t\n", utils.IsEmail("asdf@univ.ac.id"))
}
