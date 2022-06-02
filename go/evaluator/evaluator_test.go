package evaluator

import (
	"lang/lexer"
	"lang/object"
	"lang/parser"
	"reflect"
	"testing"
)

func TestEvalIntegerExpression(t *testing.T) {
	tests := []struct {
		input string
		want  int64
	}{
		{"5", 5},
		{"10", 10},
		{"-5", -5},
		{"-10", -10},
		{"5 + 5 + 5 + 5 - 10", 10},
		{"2 * 2 * 2 * 2 * 2", 32},
		{"-50 + 100 + -50", 0},
		{"5 * 2 + 10", 20},
		{"5 + 2 * 10", 25},
		{"20 + 2 * -10", 0},
		{"50 / 2 * 2 + 10", 60},
		{"2 * (5 + 10)", 30},
		{"3 * 3 * 3 + 10", 37},
		{"3 * (3 * 3) + 10", 37},
		{"(5 + 10 * 2 + 15 / 3) * 2 + -10", 50},
		{"return 10;", 10},
		{"return 10; 9;", 10},
		{"return 2 * 5; 9;", 10},
		{"9; return 2 * 5; 9;", 10},
		{`
if (10 > 1) {
if (10 > 1) {
return 10;
}
return 1;
}`, 10},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			evaluated := testEval(tt.input)
			result, _ := evaluated.(*object.Integer)
			got := result.Value
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("Eval() = %v, want %v", got, tt.want)
			}
		})
	}
}
func TestEvalBooleanExpression(t *testing.T) {
	tests := []struct {
		input string
		want  bool
	}{
		{"true", true},
		{"false", false},
		{"!true", false},
		{"!false", true},
		{"!5", false},
		{"!!true", true},
		{"!!false", false},
		{"!!5", true},
		{"1 < 2", true},
		{"1 > 2", false},
		{"1 < 1", false},
		{"1 > 1", false},
		{"1 == 1", true},
		{"1 != 1", false},
		{"1 == 2", false},
		{"1 != 2", true},
		{"true == true", true},
		{"false == false", true},
		{"true == false", false},
		{"true != false", true},
		{"false != true", true},
		{"(1 < 2) == true", true},
		{"(1 < 2) == false", false},
		{"(1 > 2) == true", false},
		{"(1 > 2) == false", true},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			evaluated := testEval(tt.input)
			result, _ := evaluated.(*object.Boolean)
			got := result.Value
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("Eval() = %v, want %v", got, tt.want)
			}
		})
	}
}
func TestIfElseExpressions(t *testing.T) {
	tests := []struct {
		input string
		want  interface{}
	}{
		{"if (true) { 10 }", 10},
		{"if (false) { 10 }", nil},
		{"if (1) { 10 }", 10},
		{"if (1 < 2) { 10 }", 10},
		{"if (1 > 2) { 10 }", nil},
		{"if (1 > 2) { 10 } else { 20 }", 20},
		{"if (1 < 2) { 10 } else { 20 }", 10},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			evaluated := testEval(tt.input)
			if tt.want == nil {
				got := evaluated
				if !reflect.DeepEqual(got, NULL) {
					t.Errorf("Eval() = %v, want %v", got, tt.want)
				}
				return
			}
			if want, ok := tt.want.(int); ok {
				got := evaluated.(*object.Integer).Value
				if !reflect.DeepEqual(got, int64(want)) {
					t.Errorf("Eval() = %v, want %v", got, tt.want)
				}
			}
		})
	}
}
func TestFunction(t *testing.T) {
	tests := []struct {
		input string
		want  interface{}
	}{
		{"func(x) { x; }(5)", 5},
		{"let identity = func(x) { x; }; identity(5);", 5},
		{"let identity = func(x) { return x; }; identity(5);", 5},
		{"let double = func(x) { x * 2; }; double(5);", 10},
		{"let add = func(x, y) { x + y; }; add(5, 5);", 10},
		{"let add = func(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			evaluated := testEval(tt.input)
			if tt.want == nil {
				got := evaluated
				if !reflect.DeepEqual(got, NULL) {
					t.Errorf("Eval() = %v, want %v", got, tt.want)
				}
				return
			}
			if want, ok := tt.want.(int); ok {
				got := evaluated.(*object.Integer).Value
				if !reflect.DeepEqual(got, int64(want)) {
					t.Errorf("Eval() = %v, want %v", got, tt.want)
				}
			}
		})
	}
}

func TestErrorHandler(t *testing.T) {
	tests := []struct {
		input string
		want  string
	}{
		{
			"5 + true;",
			"Error:type not match:INTEGER + BOOLEAN",
		},
		{
			"5 + true; 5;",
			"Error:type not match:INTEGER + BOOLEAN",
		},
		{
			"-true",
			"Error:unknown operator:-BOOLEAN",
		},
		{
			"true + false;",
			"Error:unknown operator:BOOLEAN + BOOLEAN",
		},
		{
			"5; true + false; 5",
			"Error:unknown operator:BOOLEAN + BOOLEAN",
		},
		{
			"if (10 > 1) { true + false; }",
			"Error:unknown operator:BOOLEAN + BOOLEAN",
		},
		{
			"if (10 > true) { true + false; }",
			"Error:type not match:INTEGER > BOOLEAN",
		},
		{
			"return true+false;",
			"Error:unknown operator:BOOLEAN + BOOLEAN",
		},
		{
			"return (true+false)+(true+false);",
			"Error:unknown operator:BOOLEAN + BOOLEAN",
		},
		{
			"return -(true+false);",
			"Error:unknown operator:BOOLEAN + BOOLEAN",
		},
		{`
if (10 > 1) {
if (10 > true) {
return true + false;
}
return 1;
}`,
			"Error:type not match:INTEGER > BOOLEAN",
		},
		{`
if (10 > 1) {
if (true) {
return true + false;
}
return 1;
}`,
			"Error:unknown operator:BOOLEAN + BOOLEAN",
		},
		{
			"id",
			"Error:identifier not found: id",
		},
		{`"hello" - "world"`,
			"Error:unknown operator:STRING - STRING",
		},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			evaluated := testEval(tt.input)
			got := evaluated.Inspect()
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("Eval() = %v, want %v", got, tt.want)
			}
		})
	}
}
func TestStringLiteral(t *testing.T) {
	tests := []struct {
		input string
		want  string
	}{
		{`"hello world"`,
			"hello world",
		},
		{`"hello"+" "+"world"`,
			"hello world",
		},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			evaluated := testEval(tt.input)
			got := evaluated.Inspect()
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("Eval() = %v, want %v", got, tt.want)
			}
		})
	}
}

func testEval(input string) object.Object {
	l := lexer.NewLexer(input)
	p := parser.NewParser(l)
	program := p.ProgramParser()
	env := object.NewEnvironment()
	return Eval(program, env)
}
