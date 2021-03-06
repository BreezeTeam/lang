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
		{
			`{"name": "Monkey"}[func(x) { x }];`,
			"Error:unusable as hash key: FUNCTION",
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
			`"hello world"`,
		},
		{`"hello"+" "+"world"`,
			`"hello world"`,
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

func TestArrayLiteral(t *testing.T) {
	tests := []struct {
		input string
		want  interface{}
	}{
		{"[1,2+1,3*3]",
			"[1, 3, 9]",
		},
		{"[1,2+1,3*3][0]",
			"1",
		},
		{"[1,2+1,3*3][1]",
			"3",
		},
		{"[1,2+1,3*3][3-1]",
			"9",
		},
		{
			"[1, 2, 3][0]", 1,
		},
		{
			"[1, 2, 3][1]", 2,
		},
		{
			"[1, 2, 3][2]", 3,
		},
		{
			"let i = 0; [1][i];", 1,
		},
		{
			"[1, 2, 3][1 + 1];", 3,
		},
		{
			"let myArray = [1, 2, 3]; myArray[2];", 3,
		},
		{
			"let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];", 6,
		},
		{
			"let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]", 2,
		},
		{
			"[1, 2, 3][3]",
			nil,
		},
		{
			"[1, 2, 3][-1]",
			nil,
		},
		{
			"len([])",
			0,
		},
		{
			`
					let map = func(arr, f) {
						let iter = func(arr, accumulated) {
							if (len(arr) == 0) {
								accumulated
							} else {
								iter(rest(arr), push(accumulated, f(first(arr)))); }
							};
						iter(arr, []);
					};
					let a = [1, 2, 3, 4];
					let double = func(x) { x * 2 };
					map(a, double);`,
			"[2, 4, 6, 8]",
		},
		{
			`let reduce = func(arr, initial, f) {
						let iter = func(arr, result) {
							if (len(arr) == 0) {
								result
							} else {
								iter(rest(arr), f(result, first(arr))); }
							};
							iter(arr, initial);
						};
						let sum = func(arr) {
							reduce(arr, 0, func(initial, el) { initial + el });
						};
						sum([1, 2, 3, 4, 5]);`,
			15,
		},
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
			} else {
				got := evaluated.Inspect()
				if !reflect.DeepEqual(got, tt.want) {
					t.Errorf("Eval() = %v, want %v", got, tt.want)
				}
			}
		})
	}
}

func TestBuiltinFunctions(t *testing.T) {
	tests := []struct {
		input string
		want  interface{}
	}{
		{`len("")`, 0},
		{`len("four")`, 4},
		{`len("hello world")`, 11},
		{`len(1)`, "Error:argument to `len` not supported, got INTEGER"},
		{`len("one", "two")`, "Error:wrong number of arguments. got=2, want=1"},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			evaluated := testEval(tt.input)
			if want, ok := tt.want.(int); ok {
				got := evaluated.(*object.Integer).Value
				if !reflect.DeepEqual(got, int64(want)) {
					t.Errorf("Eval() = %v, want %v", got, tt.want)
				}
			} else {
				got := evaluated.Inspect()
				if !reflect.DeepEqual(got, tt.want) {
					t.Errorf("Eval() = %v, want %v", got, tt.want)
				}
			}

		})
	}
}

func TestHashLiterals(t *testing.T) {
	tests := []struct {
		input string
		want  interface{}
	}{
		{
			`let two = "two";
				{
					"one": 10 - 9,
					two: 1 + 1,
					"thr" + "ee": 6 / 2,
					4: 4,
					true: 5,
					false: 6
				}`,
			"skip",
		},
		{
			`{"foo": 5}["foo"]`, 5,
		},
		{
			`{"foo": 5}["bar"]`,
			nil,
		},
		{
			`let key = "foo"; {"foo": 5}[key]`, 5,
		},
		{
			`{}["foo"]`,
			nil,
		},
		{
			`{5: 5}[5]`, 5,
		},
		{
			`{true: 5}[true]`, 5,
		},
		{
			`{false: 5}[false]`, 5,
		},
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
			} else {
				got := evaluated.Inspect()

				if tt.want == "skip" {
					t.Logf("Eval() = %v", got)
				} else if !reflect.DeepEqual(got, tt.want) {
					t.Errorf("Eval() = %v, want %v", got, tt.want)
				}
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
