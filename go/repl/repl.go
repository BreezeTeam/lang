package repl

import (
	"bufio"
	"fmt"
	"io"
	"lang/evaluator"
	"lang/lexer"
	"lang/object"
	"lang/parser"
)

const PROMPT = ">> "

func Start(in io.Reader, out io.Writer) {
	scanner := bufio.NewScanner(in)
	env := object.NewEnviroment()

	for {
		fmt.Printf(PROMPT)
		scanned := scanner.Scan()
		if !scanned {
			return
		}

		line := scanner.Text()
		// lexer
		l := lexer.NewLexer(line)
		// parser
		p := parser.NewParser(l)

		// parser program
		program := p.ProgramParser()

		if len(p.Errors()) != 0 {
			printParserErrors(out, p.Errors())
			continue
		}
		result := evaluator.Eval(program, env)
		if result != nil {
			io.WriteString(out, result.Inspect())
			io.WriteString(out, "\n")
		} else {
			io.WriteString(out, program.String())
			io.WriteString(out, "\n")
		}

	}
}

func printParserErrors(out io.Writer, errors []string) {
	for _, msg := range errors {
		io.WriteString(out, "\t"+msg+"\n")
	}
}
