package object

import (
	"bytes"
	"fmt"
	"hash/fnv"
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
	ARRAY_OBJ    = "ARRAY"
	HASH_OBJ     = "HASH"
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
func (s *String) Inspect() string  { return `"` + s.Value + `"` }

var _ Object = &String{}

type Builtin struct {
	Fn BuiltinFunction
}
type BuiltinFunction func(args ...Object) Object

func (b *Builtin) Type() ObjectType { return BUILTIN_OBJ }
func (b *Builtin) Inspect() string  { return "builtin function" }

var _ Object = &Builtin{}

type Array struct {
	Elements []Object
}

func (a *Array) Type() ObjectType { return ARRAY_OBJ }
func (a *Array) Inspect() string {
	var (
		out      bytes.Buffer
		elements []string
	)
	for _, e := range a.Elements {
		elements = append(elements, e.Inspect())
	}

	out.WriteString("[")
	out.WriteString(strings.Join(elements, ", "))
	out.WriteString("]")
	return out.String()
}

var _ Object = &Array{}

type HashKey struct {
	Type  ObjectType
	Value uint64
}

// Hashable 添加一种新的接口，实现了HashKey 方法的结构，为可hash结构
type Hashable interface {
	HashKey() HashKey
}

// HashKey 为 Boolean 类型添加 HashKey 方法
func (b *Boolean) HashKey() HashKey {
	var value uint64
	if b.Value {
		value = 1
	} else {
		value = 0
	}
	return HashKey{Type: b.Type(), Value: value}
}

// HashKey 为 Integer 类型添加 HashKey 方法
func (i *Integer) HashKey() HashKey {
	return HashKey{Type: i.Type(), Value: uint64(i.Value)}
}

// HashKey 为 String 类型 添加 HashKey 方法
func (s *String) HashKey() HashKey {
	h := fnv.New64a()
	h.Write([]byte(s.Value))
	return HashKey{Type: s.Type(), Value: h.Sum64()}
}

type HashPair struct {
	Key   Object
	Value Object
}

type Hash struct {
	Pairs map[HashKey]HashPair
}

func (h Hash) Type() ObjectType { return HASH_OBJ }
func (h Hash) Inspect() string {
	var (
		out   bytes.Buffer
		pairs []string
	)
	for _, pair := range h.Pairs {
		pairs = append(pairs, fmt.Sprintf("%s: %s", pair.Key.Inspect(), pair.Value.Inspect()))
	}

	out.WriteString("{")
	out.WriteString(strings.Join(pairs, ", "))
	out.WriteString("}")
	return out.String()
}

var _ Object = &Hash{}
