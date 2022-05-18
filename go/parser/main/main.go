package main

import "fmt"

type XX interface {
	xx()
}

type A struct {
	a string
}

func (x *A) xx() {}

func test2() *A {
	return nil
}
func test() XX {
	return test2()
}
func main() {
	res := test()
	if res == nil {
		fmt.Println("nil")
	} else {
		fmt.Println("not nil")
	}
}
