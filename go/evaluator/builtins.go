// Package evaluator
// @Author Euraxluo  9:21:00
package evaluator

import (
	"fmt"
	"lang/object"
)

var builtins = map[string]*object.Builtin{
	"len": &object.Builtin{
		Fn: length,
	},
	"first": &object.Builtin{
		Fn: first,
	},
	"last": &object.Builtin{
		Fn: last,
	},
	"rest": &object.Builtin{
		Fn: rest,
	},
	"push": &object.Builtin{
		Fn: push,
	},
	"print": &object.Builtin{
		Fn: output,
	},
}

func output(args ...object.Object) object.Object {
	for _, arg := range args {
		fmt.Println(arg.Inspect())
	}
	return NULL
}

func push(args ...object.Object) object.Object {
	if len(args) != 2 {
		return newError("wrong number of arguments. got=%d, want=2", len(args))
	}
	switch arg := args[0].(type) {
	case *object.Array:
		l := len(arg.Elements)
		newElements := make([]object.Object, l+1, l+1)
		copy(newElements, arg.Elements)
		newElements[l] = args[1]
		return &object.Array{Elements: newElements}
	default:
		return newError("argument to `first` not supported, got %s",
			args[0].Type())
	}
}

func rest(args ...object.Object) object.Object {
	if len(args) != 1 {
		return newError("wrong number of arguments. got=%d, want=1", len(args))
	}
	switch arg := args[0].(type) {
	case *object.Array:
		l := len(arg.Elements)
		if l > 0 {
			newElements := make([]object.Object, l-1, l-1)
			copy(newElements, arg.Elements[1:l])
			return &object.Array{Elements: newElements}
		}
		return NULL
	default:
		return newError("argument to `first` not supported, got %s",
			args[0].Type())
	}
}

func last(args ...object.Object) object.Object {
	if len(args) != 1 {
		return newError("wrong number of arguments. got=%d, want=1", len(args))
	}
	switch arg := args[0].(type) {
	case *object.Array:
		if len(arg.Elements) > 0 {
			return arg.Elements[len(arg.Elements)-1]
		}
		return NULL
	default:
		return newError("argument to `first` not supported, got %s",
			args[0].Type())
	}
}

// length the len builtin function
func length(args ...object.Object) object.Object {
	if len(args) != 1 {
		return newError("wrong number of arguments. got=%d, want=1", len(args))
	}
	switch arg := args[0].(type) {
	case *object.Array:
		return &object.Integer{Value: int64(len(arg.Elements))}
	case *object.String:
		return &object.Integer{Value: int64(len(arg.Value))}
	default:
		return newError("argument to `len` not supported, got %s",
			args[0].Type())
	}
}

// first  the first builtin function
func first(args ...object.Object) object.Object {
	if len(args) != 1 {
		return newError("wrong number of arguments. got=%d, want=1", len(args))
	}
	switch arg := args[0].(type) {
	case *object.Array:
		if len(arg.Elements) > 0 {
			return arg.Elements[0]
		}
		return NULL
	default:
		return newError("argument to `first` not supported, got %s",
			args[0].Type())
	}
}
