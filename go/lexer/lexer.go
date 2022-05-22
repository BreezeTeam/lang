package lexer

import (
	"lang/helper"
	"lang/token"
)

type Lexer struct {
	input        string
	position     int       // current position in input (points to current char)
	readPosition int       // current reading position in input (after current char)
	ch           chan Pair // channel
	char         rune
	line         int //input line with read
	linePosition int //input line_position with read
}

type Pair struct {
	Pos  int
	Char rune
}

// NewLexer  创建文本解析器
func NewLexer(input string) *Lexer {
	l := &Lexer{input: input}
	l.ch = helper.Yield(func(ts chan Pair) {
		for i, x := range l.input {
			ts <- Pair{i, x}
		}
	})
	//初始化为第一行,第-1个字符
	l.line = 1
	l.linePosition = -1
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
		// 标识符都不能以数字开头，数字开头的一定是数字
		if isDigit(l.char) {
			tok.Type = token.INT
			tok.Literal = l.readNumber() + l.readIdentifier()
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

// skipWhitespace  循环读取，并跳过空白符
func (l *Lexer) skipWhitespace() {
	for l.char == rune(' ') || l.char == rune('\t') || l.char == rune('\n') || l.char == rune('\r') {
		l.readChar()
	}
}

// readChar  如果打算读取的字符超过输入文本长度，则返回EOF，否则从channel rune流中读取字母
func (l *Lexer) readChar() {
	if l.position+len(string(l.char)) >= len(l.input) {
		l.position += len(string(l.char))
		l.char = 0
	} else {
		pair := <-l.ch
		l.char, l.position = pair.Char, pair.Pos
		if l.char == '\n' {
			l.line += 1
			l.linePosition = -1
		} else {
			l.linePosition += 1
		}
	}
	//fmt.Printf("readChar %d %c \n", l.position, l.char)
}

// peekChar  TODO:提前读取下一个字符
func (l *Lexer) peekChar() byte {
	if l.position+1 >= len(l.input) {
		return 0
	} else {
		data := l.input[l.position+1]
		return data
	}
}

// readIdentifier  循环读取字符，知道遇到一个非标识字符
func (l *Lexer) readIdentifier() string {
	position := l.position
	for isIdentifiers(l.char) {
		l.readChar()
	}
	return l.input[position:l.position]
}

// readNumber  循环读取字符,直到读取到非数字字符
func (l *Lexer) readNumber() string {
	position := l.position
	for isDigit(l.char) {
		l.readChar()
	}
	return l.input[position:l.position]
}

// isIdentifiers  判断是否为合法的标识符，即 以下划线或者字母开头， 用 字母，数字和下划线组成的 token
func isIdentifiers(ch rune) bool {
	return rune('a') <= ch && ch <= rune('z') || rune('0') <= ch && ch <= rune('9') || rune('A') <= ch && ch <= rune('Z') || ch == rune('_') || ch > 128
}

// isDigit  判断是否为数字
func isDigit(ch rune) bool {
	return rune('0') <= ch && ch <= rune('9')
}

// newToken  创建一个新token
func newToken(tokenType token.TokenType, ch rune) token.Token {
	return token.Token{Type: tokenType, Literal: string(ch)}
}
