// Package helper
// @Author Euraxluo  9:11:00
package helper

import (
	"fmt"
	"testing"
)

func Test_Yield(t *testing.T) {
	res := Yield(func(ts chan rune) {
		data := `let five = 5;
let ten = 10;

let 函数 = func(x, y) {
  x + y;
};

let result = 函数(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
`

		for i, x := range data {
			ts <- x
			fmt.Printf("%d %c \n", i, x)
		}

	})

	for x := range res {
		fmt.Printf("range value %c \n", x)
	}

}
