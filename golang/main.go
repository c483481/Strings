package main

import (
	"fmt"
	"str/utils"
)

func main() {
	fmt.Printf("result convert from \"42\": %d\n", utils.ConvertToNumber("42"))
	fmt.Printf("result convert from \"abc\": %d", utils.ConvertToNumber("abc"))
}
