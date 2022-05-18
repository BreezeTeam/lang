package ast

import (
	"bytes"
	"lang/token"
)

type Node interface {
	TokenLiteral() string
}

//
//  Statement
//  @Description: 语句
//
type Statement interface {
	Node
	statementNode()
}

//
//  Expression
//  @Description: 表达式
//
type Expression interface {
	Node
	expressionNode()
}

//
//  Program
//  @Description: 这个是lang 的 ast root node
//
type Program struct {
	//
	//  Statement
	//  @Description: 每一个有效的 ast 都是由一系列的接口  语句 组成
	//
	Statement []Statement
}

func (p *Program) TokenLiteral() (res string) {
	if len(p.Statement) > 0 {
		for _, statement := range p.Statement {
			res += statement.TokenLiteral()
		}
	}
	return
}

func (p *Program) String() string {
	var res bytes.Buffer
	for _, statement := range p.Statement {
		res.WriteString(statement.TokenLiteral())
	}
	return res.String()
}

var _ Node = &Program{}

//
//  LetStatement
//  @Description: 这是一个完整的语句，  let x = 5/ let x = func()
//
type LetStatement struct {
	Token token.Token // let
	Name  *Identifire // x
	Value Expression  // 5/func()
}

func (l *LetStatement) TokenLiteral() string {
	return l.Token.Literal
}

func (l *LetStatement) statementNode() {
	//TODO implement me
	panic("implement me")
}

func (l *LetStatement) String() string {
	return l.Token.Literal
}

var _ Statement = &LetStatement{}

//
//  Identifire
//  @Description: 这是一种表达式
//
type Identifire struct {
	Token token.Token
	Value string //这里是 token 的类型
}

func (i *Identifire) TokenLiteral() string {
	return i.Token.Literal
}

func (i *Identifire) expressionNode() {
	//TODO implement me
	panic("implement me")
}

var _ Expression = &Identifire{}
