package object

import (
	"bytes"
	"fmt"
	"lang/ast"
	"strings"
)

type ObjectType string

const (
	NULL_OBJ     = "NULL"
	INTEGER_OBJ  = "INTEGER"
	BOOLEAN_OBJ  = "BOOLEAN"
	RETURN_OBJ   = "RETURN"
	ERROR_OBJ    = "ERROR"
	FUNCTION_OBJ = "FUNCTION"
	STRING_OBJ   = "STRING"
	BUILTIN_OBJ  = "BUILTIN"
)

// Object 对象接口
type Object interface {
	Type() ObjectType //对象类型
	Inspect() string  // 对象监视值
}

// Boolean bool type object
type Boolean struct {
	Value bool
}

func (b *Boolean) Type() ObjectType { return BOOLEAN_OBJ }
func (b *Boolean) Inspect() string  { return fmt.Sprintf("%t", b.Value) }

var _ Object = &Boolean{}

// Integer int type object
type Integer struct {
	Value int64
}

func (i *Integer) Type() ObjectType { return INTEGER_OBJ }
func (i *Integer) Inspect() string  { return fmt.Sprintf("%d", i.Value) }

var _ Object = &Integer{}

// NULL type object
type NULL struct{}

func (n *NULL) Type() ObjectType { return NULL_OBJ }
func (n *NULL) Inspect() string  { return "null" }

var _ Object = &NULL{}

// Return type object
type Return struct {
	Value Object
}

func (r *Return) Type() ObjectType { return RETURN_OBJ }
func (r *Return) Inspect() string  { return r.Value.Inspect() }

var _ Object = &Return{}

// Error type object
type Error struct {
	Message string
}

func (e *Error) Type() ObjectType { return ERROR_OBJ }
func (e *Error) Inspect() string  { return "Error:" + e.Message }

var _ Object = &Error{}

// Function type object
type Function struct {
	Parameters []*ast.Identifier
	Body       *ast.BlockStatement
	Env        *Environment
}

func (f *Function) Type() ObjectType { return FUNCTION_OBJ }
func (f *Function) Inspect() string {
	var (
		out    bytes.Buffer
		params []string
	)
	for _, p := range f.Parameters {
		params = append(params, p.String())
	}
	out.WriteString("func")
	out.WriteString("(")
	out.WriteString(strings.Join(params, ", "))
	out.WriteString(")")
	out.WriteString(f.Body.String())
	return out.String()
}

var _ Object = &Function{}

type String struct {
	Value string
}

func (s *String) Type() ObjectType { return STRING_OBJ }
func (s *String) Inspect() string  { return s.Value }

var _ Object = &String{}

type Builtin struct {
	Fn BuiltinFunction
}
type BuiltinFunction func(args ...Object) Object

func (b *Builtin) Type() ObjectType { return BUILTIN_OBJ }

func (b *Builtin) Inspect() string { return "builtin function" }

var _ Object = &Builtin{}
