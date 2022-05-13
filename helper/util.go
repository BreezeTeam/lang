// Package helper
// @Author Euraxluo  9:08:00
package helper

func Yield[T any](f func(chan T)) chan T {
	ch := make(chan T)
	go func() {
		f(ch)
		close(ch)
	}()
	return ch
}
