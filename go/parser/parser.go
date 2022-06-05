package parser

import (
	"fmt"
	"lang/ast"
	"lang/lexer"
	"lang/token"
	"strconv"
)

type Parser struct {
	lexer     *lexer.Lexer
	curToken  token.Token
	nextToken token.Token

	prefixParseFuncs map[token.TokenType]prefixParseFunc
	infixParseFuncs  map[token.TokenType]infixParseFunc
	errors           []string
}

/////////////////////////////////////////////////////////////
// variable table
/////////////////////////////////////////////////////////////

type (
	//在前缀位置遇到关联的标识类型时，前缀解析将会被调用
	prefixParseFunc func() ast.Expression
	//在中缀位置遇到关联的标识类型时，中缀解析将会被调用
	infixParseFunc func(ast.Expression) ast.Expression
)

//  定义优先级
const (
	_ int = iota
	LOWEST
	EQUALS  // ==
	COMPARE // > or <
	SUM     // +
	PRODUCT // *
	PREFIX  // -X or !X
	CALL    // Function(X)
)

var Precedences = map[token.TokenType]int{
	token.EQ:       EQUALS,
	token.NOT_EQ:   EQUALS,
	token.LT:       COMPARE,
	token.GT:       COMPARE,
	token.PLUS:     SUM,
	token.MINUS:    SUM,
	token.SLASH:    PRODUCT,
	token.ASTERISK: PRODUCT,
	token.LPAREN:   CALL,
}

/////////////////////////////////////////////////////////////
// Public interface
/////////////////////////////////////////////////////////////

// NewParser   输入一个 词法分析器，返回一个语法分析器
func NewParser(l *lexer.Lexer) *Parser {
	p := &Parser{
		lexer:            l,
		prefixParseFuncs: make(map[token.TokenType]prefixParseFunc),
		infixParseFuncs:  make(map[token.TokenType]infixParseFunc),
	}
	//  注册表达式解析函数
	p.registerExpressionParseFunc()

	//推进两次，则 cur 和 next 都有token 了
	p.advanceTokens()
	p.advanceTokens()
	return p
}

// ProgramParser  解析入口
func (p *Parser) ProgramParser() *ast.Program {
	program := &ast.Program{}
	program.Statement = []ast.Statement{}
	for !p.curTokenIs(token.EOF) {
		stmt := p.parseStatement()
		if !ast.StatementIsNil(stmt) {
			program.Statement = append(program.Statement, stmt)
		}
		// TODO if stmt is nil， error handler
		p.advanceTokens()
	}
	return program
}

// Errors  错误列表
func (p *Parser) Errors() []string {
	return p.errors
}

/////////////////////////////////////////////////////////////
// Statement parse
/////////////////////////////////////////////////////////////

// parseStatement  根据Token type 进行 语句解析
func (p *Parser) parseStatement() ast.Statement {
	switch p.curToken.Type {
	case token.LET:
		return p.parseLetStatement()
	case token.RETURN:
		return p.parseReturnStatement()
	case token.LBRACE:
		return p.parseBlockStatement()
	default:
		return p.parseExpressionStatement()
	}
}

// parseLetStatement  解析let 语句
func (p *Parser) parseLetStatement() *ast.LetStatement {
	stmt := &ast.LetStatement{Token: p.curToken}
	if !p.expectNextToken(token.IDENT) {
		return nil
	}
	stmt.Name = &ast.Identifier{Token: p.curToken, Value: p.curToken.Literal}

	if !p.expectNextToken(token.ASSIGN) {
		return nil
	}
	p.advanceTokens()
	stmt.Value = p.parseExpression(LOWEST)
	if p.nextTokenIs(token.SEMICOLON) {
		p.advanceTokens()
	}
	return stmt
}

// parseReturnStatement  解析return 语句
func (p *Parser) parseReturnStatement() *ast.ReturnStatement {
	stmt := &ast.ReturnStatement{Token: p.curToken}
	p.advanceTokens()
	//获取下一个token
	stmt.ReturnValue = p.parseExpression(LOWEST)
	// ; 是可选的
	if p.nextTokenIs(token.SEMICOLON) {
		p.advanceTokens()
	}
	return stmt
}

// parseExpressionStatement  解析表达式语句
func (p *Parser) parseExpressionStatement() *ast.ExpressionStatement {
	stmt := &ast.ExpressionStatement{Token: p.curToken}
	stmt.Expression = p.parseExpression(LOWEST) // 以最低优先级进行表达式解析
	if p.nextTokenIs(token.SEMICOLON) {
		p.advanceTokens()
	}
	return stmt
}

// parseBlockStatement  解析语句块
func (p *Parser) parseBlockStatement() *ast.BlockStatement {
	block := &ast.BlockStatement{
		Token: p.curToken,
	}
	block.Statement = []ast.Statement{}
	p.advanceTokens()
	for !p.curTokenIs(token.RBRACE) && !p.curTokenIs(token.EOF) {
		stmt := p.parseStatement()
		if !ast.StatementIsNil(stmt) {
			block.Statement = append(block.Statement, stmt)
		}
		p.advanceTokens()
	}
	return block
}

/////////////////////////////////////////////////////////////
// Expression parse
/////////////////////////////////////////////////////////////

// parseExpression  根据优先级 调用合适的解析函数解析表达式
// 当-1+2+3 作为input 是，我们将进行语句解析
// 首先他会判断 是否与 `-` 这个token 相关的前缀解析函数，这时进入前缀解析，结果为`-1`
// 然后是算法中的for循环，当 下一个token的操作符优先级大于当前优先级，就会将其作为 left 解析，否则会跳过
// 最终的这个for 实现的效果是，使得高优先级的运算符比具有低优先级的运算符在树中更深
// 即如果我们的右部分的token优先级比较高，那么他永远不会成为某个节点的右字树，例如 `-1` 他将会永远成为一个单独的树
// 如果是 1+2*3，其中 由于`*` 的优先级大于`+`，这将会导致 `2` 会被传递给 `*` 作为左子树
func (p *Parser) parseExpression(precedence int) ast.Expression {
	if prefix, ok := p.prefixParseFuncs[p.curToken.Type]; ok {
		expression := prefix()

		for !p.nextTokenIs(token.SEMICOLON) && precedence < tokenTypePrecedence(p.nextToken.Type) {
			if infix, ok := p.infixParseFuncs[p.nextToken.Type]; ok {
				p.advanceTokens()
				expression = infix(expression)
			} else {
				//  如果某一个中缀表达式没有解析函数，提前退出
				return expression
			}
		}
		return expression
	} else {
		msg := fmt.Sprintf("no prefix parse function for %s found", p.curToken.Type)
		p.errors = append(p.errors, msg)
		return nil
	}
}

// parsePrefixExpression  前缀表达式解析
func (p *Parser) parsePrefixExpression() ast.Expression {
	expression := ast.PrefixExpression{
		Token:    p.curToken,
		Operator: p.curToken.Literal,
	}
	p.advanceTokens()
	expression.Right = p.parseExpression(PREFIX)
	return &expression
}

// parseInfixExpression  中缀表达式解析
func (p *Parser) parseInfixExpression(left ast.Expression) ast.Expression {
	expression := ast.InfixExpression{
		Token:    p.curToken,
		Operator: p.curToken.Literal,
		Left:     left,
	}
	currentTokenType := p.curToken.Type
	p.advanceTokens()
	expression.Right = p.parseExpression(tokenTypePrecedence(currentTokenType))
	return &expression
}

func (p *Parser) registerExpressionParseFunc() {
	p.registerPrefix(token.IDENT, p.parseIdentifiers)
	p.registerPrefix(token.INT, p.parseIntegerLiteral)
	p.registerPrefix(token.BANG, p.parsePrefixExpression)
	p.registerPrefix(token.MINUS, p.parsePrefixExpression)
	p.registerPrefix(token.PLUS, p.parsePrefixExpression)
	p.registerPrefix(token.TRUE, p.parseBoolean)
	p.registerPrefix(token.FALSE, p.parseBoolean)
	p.registerPrefix(token.LPAREN, p.parseGroupedExpression)
	p.registerPrefix(token.IF, p.parseIfExpression)
	p.registerPrefix(token.FUNCTION, p.parseFunctionLiteral)
	p.registerPrefix(token.STRING, p.parseStringLiteral)
	p.registerPrefix(token.LBRACKET, p.parseArrayLiteral)

	p.registerInfix(token.PLUS, p.parseInfixExpression)
	p.registerInfix(token.MINUS, p.parseInfixExpression)
	p.registerInfix(token.SLASH, p.parseInfixExpression)
	p.registerInfix(token.ASTERISK, p.parseInfixExpression)
	p.registerInfix(token.EQ, p.parseInfixExpression)
	p.registerInfix(token.NOT_EQ, p.parseInfixExpression)
	p.registerInfix(token.LT, p.parseInfixExpression)
	p.registerInfix(token.GT, p.parseInfixExpression)
	p.registerInfix(token.LPAREN, p.parseCallExpression)
}

// parseIdentifiers  标识符解析器
func (p *Parser) parseIdentifiers() ast.Expression {
	return &ast.Identifier{Token: p.curToken, Value: p.curToken.Literal}
}

// parseIntegerLiteral  整数表达式解析器
func (p *Parser) parseIntegerLiteral() ast.Expression {
	lit := ast.IntegerLiteral{Token: p.curToken}
	parseInt, err := strconv.ParseInt(p.curToken.Literal, 0, 64)
	if err != nil {
		msg := fmt.Sprintf("could not parse %q as integer", p.curToken.Literal)
		p.errors = append(p.errors, msg)
		return nil
	}
	lit.Value = parseInt
	return &lit
}

// parseBoolean  布尔表达式解析器
func (p *Parser) parseBoolean() ast.Expression {
	return &ast.Boolean{Token: p.curToken, Value: p.curTokenIs(token.TRUE)}
}

// parseGroupedExpression  组解析器
// 其实很简单，就是 遇到`(`,就把直到第一个遇见的`)`之间的token1 拿去解析成一个表达式
func (p *Parser) parseGroupedExpression() ast.Expression {
	//推进下一个token
	p.advanceTokens()
	expression := p.parseExpression(LOWEST)
	if !p.expectNextToken(token.RPAREN) {
		return nil
	}
	return expression
}

// parseIfExpression  解析if 表达式
func (p *Parser) parseIfExpression() ast.Expression {
	expression := &ast.IfExpression{
		Token: p.curToken,
	}
	//当前是if，应推进到（
	p.advanceTokens()
	expression.Condition = p.parseExpression(LOWEST)
	//当前是），应推进到{
	if p.expectNextToken(token.LBRACE) {
		expression.Consequence = (p.parseStatement()).(*ast.BlockStatement)
	} else {
		return expression
	}
	if p.expectNextToken(token.ELSE) {
		//下一个是），应推进到{
		if p.expectNextToken(token.LBRACE) {
			expression.Alternative = (p.parseStatement()).(*ast.BlockStatement)
		} else {
			return expression
		}
	}
	return expression
}

// parseFunctionLiteral  解析函数表达式
func (p *Parser) parseFunctionLiteral() ast.Expression {
	functionLiteral := ast.FunctionLiteral{
		Token: p.curToken,
	}
	//当前是func，推进到 （,然后进行 参数解析即 标识符列表解析
	if p.expectNextToken(token.LPAREN) {
		functionLiteral.Parameters = p.parseFuncParams()
	} else {
		return nil
	}
	//解析完成（），开始解析{}
	//推进到{
	if p.expectNextToken(token.LBRACE) {
		functionLiteral.Body = (p.parseStatement()).(*ast.BlockStatement)
	} else {
		return nil
	}

	return &functionLiteral
}

// parseFunctionLiteral  解析函数表达式
func (p *Parser) parseFuncParams() []*ast.Identifier {
	var identifier []*ast.Identifier
	//推进下一个token
	//开始时，当前的token为（
	//如果下一个是 )，则切换并退出
	if p.expectNextToken(token.RPAREN) {
		return identifier
	}
	//否则的话，开始 标识符列表的解析
	p.advanceTokens() //获取当前这一个非)的token
	identifier = append(identifier, &ast.Identifier{Token: p.curToken, Value: p.curToken.Literal})
	//然后就会遇见,;如果下一个是分隔符，那么就跳过，然后继续装载id
	for p.expectNextToken(token.COMMA) {
		p.advanceTokens() //获取当前这一个非)的token
		identifier = append(identifier, &ast.Identifier{Token: p.curToken, Value: p.curToken.Literal})
	}
	//此时当前token是最后一个标识符
	//跳过最后的),如果没有)，则有问题，应该报错
	if !p.expectNextToken(token.RPAREN) {
		return nil
	}
	return identifier
}

// parseCallExpression returns the next token
// function 被调用的函数，同时这里还有一种情况，那就是 function 这个地方不是一个标识符，还是一个 expression
// call 本质上是一个 中缀表达式
// left 为 一个标识符，表示一个函数
// right 为 参数列表，传递给 函数
func (p *Parser) parseCallExpression(function ast.Expression) ast.Expression {
	expression := &ast.CallExpression{
		Token:    p.curToken,
		Function: function,
	}
	// 此时 当前 token 为？
	expression.Arguments = p.parseExpressionList(token.RPAREN)
	return expression
}

// parseStringLiteral 解析String 字面量
func (p *Parser) parseStringLiteral() ast.Expression {
	return &ast.StringLiteral{Token: p.curToken, Value: p.curToken.Literal}
}

// parseArrayLiteral  解析array
func (p *Parser) parseArrayLiteral() ast.Expression {
	array := ast.ArrayLiteral{Token: p.curToken}
	array.Elements = p.parseExpressionList(token.RBRACKET)
	return &array
}

// parseExpressionList  解析表达式列表
func (p *Parser) parseExpressionList(end token.TokenType) []ast.Expression {
	var list []ast.Expression
	//推进下一个token
	//开始时，当前的token为（
	//如果下一个是 )，则切换并退出
	if p.expectNextToken(end) {
		p.advanceTokens()
		return list
	}
	//否则的话，开始 调用参数列表的解析
	p.advanceTokens() //获取当前这一个非)的token

	// 从下一个token 开始解析 表达式
	list = append(list, p.parseExpression(LOWEST))
	//然后就会遇见,;如果下一个是分隔符，那么就跳过，然后继续装载expression
	for p.expectNextToken(token.COMMA) {
		p.advanceTokens() //获取当前这一个非)的token
		list = append(list, p.parseExpression(LOWEST))
	}
	//此时当前token是最后一个标识符
	//跳过最后的),如果没有)，则有问题，应该报错
	if !p.expectNextToken(end) {
		return nil
	}
	return list
}

/////////////////////////////////////////////////////////////
// helper
/////////////////////////////////////////////////////////////

// advanceTokens  token 推进器，将下一个token设置为当前token，并获取下一个token
func (p *Parser) advanceTokens() { // advanceTokens
	p.curToken = p.nextToken
	p.nextToken = p.lexer.NextToken()
}

// expectNextToken  判断下一个token是不是期望的token，如果是，则推进获取并返回true；反之返回false
func (p *Parser) expectNextToken(tokenType token.TokenType) bool {
	if p.nextTokenIs(tokenType) {
		p.advanceTokens()
		return true
	} else {
		return false
	}
}

// nextTokenIs  输入 tokenType 判断下一个token 是不是此类型
func (p *Parser) nextTokenIs(tokenType token.TokenType) bool {
	return p.nextToken.Type == tokenType
}

// curTokenIs  输入 tokenType 判断 当前 token 是不是此类型
func (p *Parser) curTokenIs(tokenType token.TokenType) bool {
	return p.curToken.Type == tokenType
}

// registerPrefix  注册前缀解析函数
func (p *Parser) registerPrefix(tokenType token.TokenType, fn prefixParseFunc) {
	p.prefixParseFuncs[tokenType] = fn
}

// registerInfix  注册中缀解析函数
func (p *Parser) registerInfix(tokenType token.TokenType, fn infixParseFunc) {
	p.infixParseFuncs[tokenType] = fn
}

// tokenTypePrecedence  根据token类型，返回优先级
func tokenTypePrecedence(tokenType token.TokenType) int {
	if p, ok := Precedences[tokenType]; ok {
		return p
	}
	return LOWEST
}

/////////////////////////////////////////////////////////////
// error handler
/////////////////////////////////////////////////////////////
