package parser

import (
	"lang/ast"
	"lang/lexer"
	"lang/token"
)

type Parser struct {
	lexer     *lexer.Lexer
	curToken  token.Token
	nextToken token.Token
}

//
//  NewParser
//  @Description: 输入一个 词法分析器，返回一个语法分析器
//  @param l
//  @return *Parser
//
func NewParser(l *lexer.Lexer) *Parser {
	p := &Parser{
		lexer: l,
	}
	//推进两次，则 cur 和 next 都有token 了
	p.AdvanceTokens()
	p.AdvanceTokens()
	return p
}

//
//  AdvanceTokens
//  @Description: token 推进器，将下一个token设置为当前token，并获取下一个token
//  @receiver l
//
func (p *Parser) AdvanceTokens() {
	p.curToken = p.nextToken
	p.nextToken = p.lexer.NextToken()
}

//
//  ProgramParser
//  @Description: 递归栈底
//  @receiver l
//
func (p *Parser) ProgramParser() *ast.Program {
	program := &ast.Program{}
	program.Statement = []ast.Statement{}
	for !p.curTokenIs(token.EOF) {
		stmt := p.parseStatement()
		if stmt != nil {
			//TODO 类型有问题吧
			program.Statement = append(program.Statement, stmt)
		}
		p.AdvanceTokens()
	}
	return program
}

//
//  parseStatement
//  @Description: 根据Token type 进行 语句解析
//  @receiver p
//  @return ast.Statement  返回接口
//
func (p *Parser) parseStatement() ast.Statement {
	switch p.curToken.Type {
	case token.LET:
		return p.parseLetStatement()
	default:
		return nil
	}
}

//
//  parseLetStatement
//  @Description: 解析let 语句
//  @receiver p
//  @return ast.Statement
//
func (p *Parser) parseLetStatement() *ast.LetStatement {
	stmt := &ast.LetStatement{Token: p.curToken}
	if !p.expectNextToken(token.IDENT) {
		return nil
	}
	stmt.Name = &ast.Identifire{Token: p.curToken, Value: p.curToken.Literal}

	if !p.expectNextToken(token.ASSIGN) {
		return nil
	}
	if !p.curTokenIs(token.SEMICOLON) {
		p.AdvanceTokens()
	}
	return stmt
}

//
//  expectNextToken
//  @Description: 判断下一个token是不是期望的token，如果是，则推进获取并返回true；反之返回false
//  @receiver p
//  @param tokenType
//  @return bool
//
func (p *Parser) expectNextToken(tokenType token.TokenType) bool {
	if p.nextTokenIs(tokenType) {
		p.AdvanceTokens()
		return true
	} else {
		return false
	}
}

//
//  nextTokenIs
//  @Description: 输入 tokenType 判断下一个token 是不是此类型
//  @receiver p
//  @param tokenType
//  @return bool
//
func (p *Parser) nextTokenIs(tokenType token.TokenType) bool {
	return p.nextToken.Type == tokenType
}

//
//  curTokenIs
//  @Description: 输入 tokenType 判断 当前 token 是不是此类型
//  @receiver p
//  @param tokenType
//  @return bool
//
func (p *Parser) curTokenIs(tokenType token.TokenType) bool {
	return p.curToken.Type == tokenType
}
