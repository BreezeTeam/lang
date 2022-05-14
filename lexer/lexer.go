package lexer

import (
	"fmt"
	"lang/helper"
	"lang/token"
)

type Lexer struct {
	input        string
	position     int       // current position in input (points to current char)
	readPosition int       // current reading position in input (after current char)
	ch           chan Pair // channel
	char         rune
}
type Pair struct {
	Pos  int
	Char rune
}

//
//  NewLexer
//  @Description: 该处初始化输入的string 应当就是一个完全的程序了
//  @param input
//  @return *Lexer
//
func NewLexer(input string) *Lexer {
	l := &Lexer{input: input}
	l.ch = helper.Yield(func(ts chan Pair) {
		for i, x := range l.input {
			ts <- Pair{i, x}
		}
	})
	l.readChar()
	return l
}

func (l *Lexer) NextToken() token.Token {
	var tok token.Token

	l.skipWhitespace()

	switch l.char {
	case '=':
		if l.peekChar() == '=' {
			char := l.char
			l.readChar()
			literal := string(char) + string(l.char)
			tok = token.Token{Type: token.EQ, Literal: literal}
		} else {
			tok = newToken(token.ASSIGN, l.char)
		}
	case '+':
		tok = newToken(token.PLUS, l.char)
	case '-':
		tok = newToken(token.MINUS, l.char)
	case '!':
		if l.peekChar() == '=' {
			char := l.char
			l.readChar()
			literal := string(char) + string(l.char)
			tok = token.Token{Type: token.NOT_EQ, Literal: literal}
		} else {
			tok = newToken(token.BANG, l.char)
		}
	case '/':
		tok = newToken(token.SLASH, l.char)
	case '*':
		tok = newToken(token.ASTERISK, l.char)
	case '<':
		tok = newToken(token.LT, l.char)
	case '>':
		tok = newToken(token.GT, l.char)
	case ';':
		tok = newToken(token.SEMICOLON, l.char)
	case ',':
		tok = newToken(token.COMMA, l.char)
	case '{':
		tok = newToken(token.LBRACE, l.char)
	case '}':
		tok = newToken(token.RBRACE, l.char)
	case '(':
		tok = newToken(token.LPAREN, l.char)
	case ')':
		tok = newToken(token.RPAREN, l.char)
	case 0:
		tok.Literal = ""
		tok.Type = token.EOF
	default:
		if isDigit(l.char) {
			tok.Type = token.INT
			tok.Literal = l.readNumber()
			return tok
		} else if isIdentifiers(l.char) {
			tok.Literal = l.readIdentifier()
			tok.Type = token.LookupIdent(tok.Literal)
			return tok
		} else {
			tok = newToken(token.ILLEGAL, l.char)
		}
	}

	l.readChar()
	return tok
}

func (l *Lexer) skipWhitespace() {
	for l.char == rune(' ') || l.char == rune('\t') || l.char == rune('\n') || l.char == rune('\r') {
		l.readChar()
	}
}
func (l *Lexer) readChar() {
	if l.position >= len(l.input) {
		l.char = 0
	} else {
		pair := <-l.ch
		l.char, l.position = pair.Char, pair.Pos
	}
	fmt.Printf("%d %c \n", l.position, l.char)
}

func (l *Lexer) peekChar() byte {
	if l.position+1 >= len(l.input) {
		return 0
	} else {
		data := l.input[l.position+1]
		return data
	}
}

func (l *Lexer) readIdentifier() string {
	position := l.position
	for isIdentifiers(l.char) {
		l.readChar()
	}
	return l.input[position:l.position]
}

func (l *Lexer) readNumber() string {
	position := l.position
	for isDigit(l.char) {
		l.readChar()
	}
	return l.input[position:l.position]
}

func isIdentifiers(ch rune) bool {
	return rune('a') <= ch && ch <= rune('z') || rune('A') <= ch && ch <= rune('Z') || ch == rune('_') || ch > 128
}
func isDigit(ch rune) bool {
	return rune('0') <= ch && ch <= rune('9')
}

func newToken(tokenType token.TokenType, ch rune) token.Token {
	return token.Token{Type: tokenType, Literal: string(ch)}
}
