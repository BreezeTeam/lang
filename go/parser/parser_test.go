package parser

import (
	"lang/lexer"
	"testing"
)

//
//  TestParser_ProgramParser
//  @Description: 正确的测试方法是构建 词法解析器，通过确定性的结果进行语法分析器测试
//  @param t
//
func TestParser_ProgramParser(t *testing.T) {
	tests := []struct {
		input         string
		wantStatement int
	}{
		{
			"let",
			0,
		},
		{
			"let x",
			0,
		},
		{
			"let x+y",
			0,
		},
		{
			"let x=",
			0,
		},
		{
			"let x=1",
			1,
		},
		{
			"let x=1;",
			1,
		},
		{
			`let x=5;let y=6;let xx = x+y;`,
			3,
		},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			programe := NewParser(lexer.NewLexer(tt.input)).ProgramParser()
			if programe == nil {
				t.Errorf("`%s` parser result is nil", tt.input)
			} else {
				if len(programe.Statement) != tt.wantStatement {
					t.Errorf("`%s` parser Statement is %d ,want %d", tt.input, len(programe.Statement), tt.wantStatement)
				}
			}
		})
	}
}
