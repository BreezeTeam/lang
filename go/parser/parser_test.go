package parser

import (
	"lang/lexer"
	"testing"
)

func TestLetStatements(t *testing.T) {
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
			"let x=12DSADAS;",
			1,
		},
		{
			"let x=y;",
			1,
		},
		{
			`let x=5;let y=6;let xx = x+y;`,
			3,
		},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			p := NewParser(lexer.NewLexer(tt.input))
			program := p.ProgramParser()
			checkParserErrors(t, p)
			if program == nil {
				t.Errorf("`%s` parser result is nil", tt.input)
			} else {
				t.Log(program.String())
				if len(program.Statement) != tt.wantStatement {
					t.Errorf("`%s` parser Statement is %d ,want %d", tt.input, len(program.Statement), tt.wantStatement)
				}
			}
		})
	}
}
func TestReturnStatements(t *testing.T) {
	tests := []struct {
		input         string
		wantStatement int
	}{
		{
			"return",
			0,
		},
		{
			"return 0",
			0,
		},
		{
			"return asa",
			0,
		},
		{
			"return as;",
			1,
		},
		{
			"return;",
			0,
		},
		{
			"return abc(1231)",
			0,
		},
		{
			"return abc(1231);",
			1,
		},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			p := NewParser(lexer.NewLexer(tt.input))
			program := p.ProgramParser()
			checkParserErrors(t, p)
			if program == nil {
				t.Errorf("`%s` parser result is nil", tt.input)
			} else {
				t.Log(program.String())
				if len(program.Statement) != tt.wantStatement {
					t.Errorf("`%s` parser Statement is %d ,want %d", tt.input, len(program.Statement), tt.wantStatement)
				}
			}
		})
	}
}

func TestIdentifierExpression(t *testing.T) {
	tests := []struct {
		input         string
		wantStatement int
	}{
		{
			"abc",
			1,
		},
		{
			"abc;",
			1,
		},
		{
			"123abc;",
			1,
		},
		{
			"abc123;",
			1,
		},
		{
			"_abc123;",
			1,
		},
		{
			"_abc中文123;",
			1,
		},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			p := NewParser(lexer.NewLexer(tt.input))
			program := p.ProgramParser()
			checkParserErrors(t, p)
			if program == nil {
				t.Errorf("`%s` parser result is nil", tt.input)
			} else {
				t.Log(program.String())
				if len(program.Statement) != tt.wantStatement {
					t.Errorf("`%s` parser Statement is %d ,want %d", tt.input, len(program.Statement), tt.wantStatement)
				}
			}
		})
	}
}

func TestIntegerLiteralExpression(t *testing.T) {
	tests := []struct {
		input         string
		wantStatement int
	}{
		{
			"123",
			1,
		},
		{
			"123abc;",
			0,
		},
		{
			"123;",
			1,
		},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			p := NewParser(lexer.NewLexer(tt.input))
			program := p.ProgramParser()
			checkParserErrors(t, p)
			if program == nil {
				t.Errorf("`%s` parser result is nil", tt.input)
			} else {
				t.Log(program.String())
				if len(program.Statement) != tt.wantStatement {
					t.Errorf("`%s` parser Statement is %d ,want %d", tt.input, len(program.Statement), tt.wantStatement)
				}
			}
		})
	}
}

func TestParsingPrefixExpressions(t *testing.T) {
	tests := []struct {
		input         string
		wantStatement int
	}{
		{
			"!5",
			1,
		},
		{
			"-5;",
			1,
		},
		{
			"+5;",
			1,
		},
		{
			"!dsa;",
			1,
		},
		{
			"-dsa;",
			1,
		},
		{
			"!true;",
			1,
		},
		{
			"!_sa中文;",
			1,
		},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			p := NewParser(lexer.NewLexer(tt.input))
			program := p.ProgramParser()
			checkParserErrors(t, p)
			if program == nil {
				t.Errorf("`%s` parser result is nil", tt.input)
			} else {
				t.Log(program.String())
				if len(program.Statement) != tt.wantStatement {
					t.Errorf("`%s` parser Statement is %d ,want %d", tt.input, len(program.Statement), tt.wantStatement)
				}
			}
		})
	}
}

func TestParsingInfixExpressions(t *testing.T) {
	tests := []struct {
		input         string
		left          int
		operator      string
		right         int
		wantStatement int
	}{
		{"5 + 5;", 5, "+", 5, 1},
		{"5 - 5;", 5, "-", 5, 1},
		{"5 * 5;", 5, "*", 5, 1},
		{"5 / 5;", 5, "/", 5, 1},
		{"5 > 5;", 5, ">", 5, 1},
		{"5 < 5;", 5, "<", 5, 1},
		{"5 == 5;", 5, "==", 5, 1},
		{"5 != 5;", 5, "!=", 5, 1},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			p := NewParser(lexer.NewLexer(tt.input))
			program := p.ProgramParser()
			checkParserErrors(t, p)
			if program == nil {
				t.Errorf("`%s` parser result is nil", tt.input)
			} else {
				t.Log(program.String())
				if len(program.Statement) != tt.wantStatement {
					t.Errorf("`%s` parser Statement is %d ,want %d", tt.input, len(program.Statement), tt.wantStatement)
				}
			}
		})
	}
}

func TestParsingBooleanExpressions(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{
			"-a * b",
			"((-a) * b)",
		},
		{
			"!-a",
			"(!(-a))",
		},
		{
			"a + b + c",
			"((a + b) + c)",
		},
		{
			"a + b - c",
			"((a + b) - c)",
		},
		{
			"a * b * c",
			"((a * b) * c)",
		},
		{
			"a * b / c",
			"((a * b) / c)",
		},
		{
			"a + b / c",
			"(a + (b / c))",
		},
		{
			"a + b * c + d / e - f",
			"(((a + (b * c)) + (d / e)) - f)",
		},
		{
			"3 + 4; -5 * 5",
			"(3 + 4)((-5) * 5)",
		},
		{
			"5 > 4 == 3 < 4",
			"((5 > 4) == (3 < 4))",
		},
		{
			"5 < 4 != 3 > 4",
			"((5 < 4) != (3 > 4))",
		},
		{
			"3 + 4 * 5 == 3 * 1 + +4 * 5",
			"((3 + (4 * 5)) == ((3 * 1) + ((+4) * 5)))",
		},
		{
			"3 + 4 * 5 == 3 * 1 + 4 * 5",
			"((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
		},
		{
			"let x = 1 * 2 + 1 == 1 + 1 * 2",
			"let x = (((1 * 2) + 1) == (1 + (1 * 2)));",
		},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			p := NewParser(lexer.NewLexer(tt.input))
			program := p.ProgramParser()
			checkParserErrors(t, p)
			if program == nil {
				t.Errorf("`%s` parser result is nil", tt.input)
			} else {
				if program.String() != tt.expected {
					t.Errorf("`%s` parser Statement is %s ,want %s", tt.input, program.String(), tt.expected)
				} else {
					t.Logf("`%s` => `%s`", tt.input, program.String())
				}
			}
		})
	}
}
func TestOperatorPrecedenceParsing(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{
			"true",
			"true",
		},
		{
			"false",
			"false",
		},
		{
			"3 > 5 == false",
			"((3 > 5) == false)",
		},
		{
			"3 < 5 == true",
			"((3 < 5) == true)",
		},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			p := NewParser(lexer.NewLexer(tt.input))
			program := p.ProgramParser()
			checkParserErrors(t, p)
			if program == nil {
				t.Errorf("`%s` parser result is nil", tt.input)
			} else {
				if program.String() != tt.expected {
					t.Errorf("`%s` parser Statement is %s ,want %s", tt.input, program.String(), tt.expected)
				} else {
					t.Logf("`%s` => `%s`", tt.input, program.String())
				}
			}
		})
	}
}

func TestOperatorPrecedenceWithGroupedParsing(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{
			"1 + (2 + 3) + 4",
			"((1 + (2 + 3)) + 4)",
		},
		{
			"(a)",
			"a",
		},
		{
			"(5 + 5) * 2",
			"((5 + 5) * 2)",
		},
		{
			"2 / (5 + 5)",
			"(2 / (5 + 5))",
		},
		{
			"-(5 + 5)",
			"(-(5 + 5))",
		},
		{
			"!(true == true)",
			"(!(true == true))",
		},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			p := NewParser(lexer.NewLexer(tt.input))
			program := p.ProgramParser()
			checkParserErrors(t, p)
			if program == nil {
				t.Errorf("`%s` parser result is nil", tt.input)
			} else {
				if program.String() != tt.expected {
					t.Errorf("`%s` parser Statement is %s ,want %s", tt.input, program.String(), tt.expected)
				} else {
					t.Logf("`%s` => `%s`", tt.input, program.String())
				}
			}
		})
	}
}

func TestParseIfExpression(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{
			"if (x<5){x}else{y}",
			"if (x < 5) { x } else { y } ",
		},
		{
			"if x<5{x}else{y}",
			"if (x < 5) { x } else { y } ",
		},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			p := NewParser(lexer.NewLexer(tt.input))
			program := p.ProgramParser()
			checkParserErrors(t, p)
			if program == nil {
				t.Errorf("`%s` parser result is nil", tt.input)
			} else {
				if program.String() != tt.expected {
					t.Errorf("`%s` parser Statement is %s ,want %s", tt.input, program.String(), tt.expected)
				} else {
					t.Logf("`%s` => `%s`", tt.input, program.String())
				}
			}
		})
	}
}

// checkParserErrors  验证有没有解析错误
func checkParserErrors(t *testing.T, p *Parser) {
	errors := p.Errors()
	if len(errors) == 0 {
		return
	}

	t.Errorf("parser has %d errors", len(errors))
	for _, msg := range errors {
		t.Errorf("parser error: %q", msg)
	}
	t.FailNow()
}
