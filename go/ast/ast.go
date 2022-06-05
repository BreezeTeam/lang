package ast

import (
	"bytes"
	"lang/token"
	"reflect"
	"strings"
)

type Node interface {
	TokenLiteral() string
	String() string
}

// Statement  语句接口
type Statement interface {
	Node
	statementNode()
}

// StatementIsNil  判断一个 语句接口是否是nil
func StatementIsNil(statement Statement) bool {
	if statement == nil || reflect.ValueOf(statement).IsNil() {
		return true
	}
	return false
}

// Expression  表达式接口
type Expression interface {
	Node
	expressionNode()
}

//////////////////////////////////////////
// ast Node 结构体
//////////////////////////////////////////

// Program  这个是lang 的 ast root node
type Program struct {
	// Statement  每一个有效的 ast 都是由一系列的接口  语句 组成
	Statement []Statement
}

func (p *Program) String() string {
	var out bytes.Buffer
	for _, s := range p.Statement {
		out.WriteString(s.String())
	}
	return out.String()
}

func (p *Program) TokenLiteral() (res string) {
	if len(p.Statement) > 0 {
		for _, statement := range p.Statement {
			res += statement.TokenLiteral()
		}
	}
	return
}

var _ Node = &Program{}

//////////////////////////////////////////
// Literal
//////////////////////////////////////////

type IntegerLiteral struct {
	Token token.Token
	Value int64
}

func (i *IntegerLiteral) TokenLiteral() string {
	return i.Token.Literal
}

func (i *IntegerLiteral) String() string {
	return i.Token.Literal
}

func (i *IntegerLiteral) expressionNode() {
	//TODO implement me
	panic("implement me")
}

var _ Expression = &IntegerLiteral{}

type Boolean struct {
	Token token.Token
	Value bool
}

func (b *Boolean) TokenLiteral() string {
	return b.Token.Literal
}

func (b *Boolean) String() string {
	return b.Token.Literal
}

func (b *Boolean) expressionNode() {
	//TODO implement me
	panic("implement me")
}

var _ Expression = &Boolean{}

type FunctionLiteral struct {
	Token      token.Token // func token
	Parameters []*Identifier
	Body       *BlockStatement
}

func (f *FunctionLiteral) TokenLiteral() string {
	return f.Token.Literal
}

func (f *FunctionLiteral) String() string {
	var (
		out    bytes.Buffer
		params []string
	)
	for _, p := range f.Parameters {
		params = append(params, p.String())
	}
	out.WriteString(f.TokenLiteral())
	out.WriteString("(")
	out.WriteString(strings.Join(params, ", "))
	out.WriteString(")")
	out.WriteString(f.Body.String())
	return out.String()
}

func (f *FunctionLiteral) expressionNode() {
	//TODO implement me
	panic("implement me")
}

var _ Expression = &FunctionLiteral{}

type StringLiteral struct {
	Token token.Token
	Value string
}

func (s *StringLiteral) expressionNode() {}
func (s *StringLiteral) TokenLiteral() string {
	return s.Token.Literal
}

func (s *StringLiteral) String() string {
	return s.Token.Literal
}

var _ Expression = &StringLiteral{}

type ArrayLiteral struct {
	Token    token.Token
	Elements []Expression
}

func (a *ArrayLiteral) TokenLiteral() string {
	return a.Token.Literal
}

func (a *ArrayLiteral) String() string {
	var out bytes.Buffer
	elements := []string{}
	for _, el := range a.Elements {
		elements = append(elements, el.String())
	}
	out.WriteString("[")
	out.WriteString(strings.Join(elements, ","))
	out.WriteString("]")
	return out.String()
}

func (a *ArrayLiteral) expressionNode() {
	//TODO implement me
	panic("implement me")
}

var _ Expression = &ArrayLiteral{}

//////////////////////////////////////////
// 表达式 结构
//////////////////////////////////////////

// Identifier  标识符表达式
type Identifier struct {
	Token token.Token
	Value string //这里是 token 的类型
}

func (i *Identifier) String() string {
	return i.Value
}

func (i *Identifier) TokenLiteral() string {
	return i.Token.Literal
}

func (i *Identifier) expressionNode() {
	//TODO implement me
	panic("implement me")
}

var _ Expression = &Identifier{}

type PrefixExpression struct {
	Token    token.Token
	Operator string
	Right    Expression
}

func (p *PrefixExpression) TokenLiteral() string {
	return p.Token.Literal
}

func (p *PrefixExpression) String() string {
	var out bytes.Buffer
	out.WriteString("(")
	out.WriteString(p.Operator)
	out.WriteString(p.Right.String())
	out.WriteString(")")
	return out.String()
}

func (p PrefixExpression) expressionNode() {
	//TODO implement me
	panic("implement me")
}

var _ Expression = &PrefixExpression{}

type InfixExpression struct {
	Token    token.Token
	Left     Expression
	Operator string
	Right    Expression
}

func (i *InfixExpression) TokenLiteral() string {
	return i.Token.Literal
}

func (i *InfixExpression) String() string {
	var out bytes.Buffer
	out.WriteString("(")
	out.WriteString(i.Left.String())
	out.WriteString(" " + i.Operator + " ")
	out.WriteString(i.Right.String())
	out.WriteString(")")
	return out.String()
}

func (i *InfixExpression) expressionNode() {
	//TODO implement me
	panic("implement me")
}

var _ Expression = &InfixExpression{}

type IfExpression struct {
	Token       token.Token
	Condition   Expression
	Consequence *BlockStatement
	Alternative *BlockStatement
}

func (i *IfExpression) TokenLiteral() string {
	return i.Token.Literal
}

func (i *IfExpression) String() string {
	var out bytes.Buffer
	out.WriteString("if ")
	out.WriteString(i.Condition.String())
	out.WriteString(i.Consequence.String())
	if i.Alternative != nil {
		out.WriteString("else")
		out.WriteString(i.Alternative.String())
	}
	return out.String()
}

func (i *IfExpression) expressionNode() {
	//TODO implement me
	panic("implement me")
}

var _ Expression = &IfExpression{}

type CallExpression struct {
	Token     token.Token // token () is call
	Function  Expression
	Arguments []Expression
}

func (c *CallExpression) TokenLiteral() string {
	return c.Token.Literal
}

func (c *CallExpression) String() string {
	var (
		out  bytes.Buffer
		args []string
	)
	for _, a := range c.Arguments {
		args = append(args, a.String())
	}

	out.WriteString(c.Function.String())
	out.WriteString("(")
	out.WriteString(strings.Join(args, ", "))
	out.WriteString(")")
	return out.String()
}

func (c *CallExpression) expressionNode() {
	//TODO implement me
	panic("implement me")
}

var _ Expression = &CallExpression{}

//////////////////////////////////////////
// 语句 结构
//////////////////////////////////////////

// LetStatement  let 语句  let x = 5/ let x = func()
type LetStatement struct {
	Token token.Token // let
	Name  *Identifier // x
	Value Expression  // 5/func()
}

func (l *LetStatement) String() string {
	var out bytes.Buffer
	out.WriteString(l.TokenLiteral() + " ")
	out.WriteString(l.Name.String())
	out.WriteString(" = ")
	if l.Value != nil {
		out.WriteString(l.Value.String())
	}
	out.WriteString(";")
	return out.String()
}

func (l *LetStatement) TokenLiteral() string {
	return l.Token.Literal
}

func (l *LetStatement) statementNode() {
	//TODO implement me
	panic("implement me")
}

var _ Statement = &LetStatement{}

// ReturnStatement  return 语句
type ReturnStatement struct {
	Token       token.Token // return token
	ReturnValue Expression  // return value
}

func (r *ReturnStatement) String() string {
	var out bytes.Buffer
	out.WriteString(r.Token.Literal + " ")
	if r.ReturnValue != nil {
		out.WriteString(r.ReturnValue.String())
	}
	out.WriteString(";")
	return out.String()
}

func (r *ReturnStatement) TokenLiteral() string {
	return r.Token.Literal
}

func (r *ReturnStatement) statementNode() {
	//TODO implement me
	panic("implement me")
}

var _ Statement = &ReturnStatement{}

// ExpressionStatement  表达式语句
type ExpressionStatement struct {
	Token      token.Token
	Expression Expression
}

func (e *ExpressionStatement) String() string {
	if e.Expression != nil {
		return e.Expression.String()
	}
	return ""
}

func (e *ExpressionStatement) TokenLiteral() string {
	return e.Token.Literal
}

func (e *ExpressionStatement) statementNode() {
	//TODO implement me
	panic("implement me")
}

var _ Statement = &ExpressionStatement{}

type BlockStatement struct {
	Token     token.Token // {
	Statement []Statement // 块语句类似于一个作用域内的脚本
}

func (b *BlockStatement) TokenLiteral() string {
	return b.Token.Literal
}

func (b *BlockStatement) String() string {
	var out bytes.Buffer
	out.WriteString(" { ")
	for _, s := range b.Statement {
		out.WriteString(s.String())
	}
	out.WriteString(" } ")
	return out.String()
}

func (b *BlockStatement) statementNode() {
	//TODO implement me
	panic("implement me")
}

var _ Statement = &BlockStatement{}
