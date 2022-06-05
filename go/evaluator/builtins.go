// Package evaluator
// @Author Euraxluo  9:21:00
package evaluator

import "lang/object"

func length(args ...object.Object) object.Object {
	if len(args) != 1 {
		return newError("wrong number of arguments. got=%d, want=1", len(args))
	}
	switch arg := args[0].(type) {
	case *object.String:
		return &object.Integer{Value: int64(len(arg.Value))}
	default:
		return newError("argument to `len` not supported, got %s",
			args[0].Type())
	}
}

var builtins = map[string]*object.Builtin{
	"len": &object.Builtin{
		Fn: length,
	},
}
