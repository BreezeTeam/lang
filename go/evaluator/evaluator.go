package evaluator

import (
	"fmt"
	"lang/ast"
	"lang/object"
	"lang/token"
)

var (
	NULL  = &object.NULL{}
	TRUE  = &object.Boolean{Value: true}
	FALSE = &object.Boolean{Value: false}
)

// isError  判断是否是Error类型
func isError(obj object.Object) bool {
	if obj != nil {
		return obj.Type() == object.ERROR_OBJ
	}
	return false
}

// isRealTrue  判断object 类型的真正真假
func isRealTrue(obj object.Object) bool {
	switch obj {
	case NULL:
		return false
	case TRUE:
		return true
	case FALSE:
		return false
	default:
		return true
	}
}

// newError  构造错误
func newError(s string, a ...interface{}) object.Object {
	return &object.Error{Message: fmt.Sprintf(s, a...)}
}

/////////////////////////////////
// eval functions
////////////////////////////////

// Eval tree-walking algo run ast tree
func Eval(node ast.Node) object.Object {
	switch node := node.(type) {
	case *ast.Program:
		result := evalStatements(node.Statement)
		if returnValue, ok := result.(*object.Return); ok {
			return returnValue.Value
		}
		return result
	case *ast.ExpressionStatement:
		return Eval(node.Expression)
	case *ast.IntegerLiteral:
		return &object.Integer{Value: node.Value}
	case *ast.Boolean:
		return nativeBoolToBooleanObject(node.Value)
	case *ast.PrefixExpression:
		return evalPrefixExpression(node)
	case *ast.InfixExpression:
		return evalInfixExpression(node)
	case *ast.IfExpression:
		return evalIfExpression(node)
	case *ast.BlockStatement:
		return evalStatements(node.Statement)
	case *ast.ReturnStatement:
		return &object.Return{Value: Eval(node.ReturnValue)}

	}
	return nil
}

// evalIfExpression  eval if语句
func evalIfExpression(node *ast.IfExpression) object.Object {
	condition := Eval(node.Condition)
	if isError(condition) {
		return condition
	} else if isRealTrue(condition) {
		return Eval(node.Consequence)
	} else if node.Alternative != nil {
		return Eval(node.Alternative)
	} else {
		return NULL
	}
}

// evalInfixExpression  eval infix
func evalInfixExpression(node *ast.InfixExpression) object.Object {
	left := Eval(node.Left)
	if isError(left) {
		return left
	}
	right := Eval(node.Right)
	if isError(right) {
		return right
	}
	switch {
	case left.Type() == object.INTEGER_OBJ && right.Type() == object.INTEGER_OBJ:
		return evalIntegerInfixExpression(node.Operator, left, right)
	case token.EQ == node.Operator:
		return nativeBoolToBooleanObject(left == right)
	case token.NOT_EQ == node.Operator:
		return nativeBoolToBooleanObject(left != right)
	case left.Type() != right.Type():
		return newError("type not match:%s %s %s", left.Type(), node.Operator, right.Type())
	default:
		return newError("unknown operator:%s %s %s", left.Type(), node.Operator, right.Type())
	}
}

// evalIntegerInfixExpression  eval integer
func evalIntegerInfixExpression(operator string, left object.Object, right object.Object) object.Object {
	leftVal := left.(*object.Integer).Value
	rightVal := right.(*object.Integer).Value
	switch operator {
	case token.PLUS:
		return &object.Integer{Value: leftVal + rightVal}
	case token.MINUS:
		return &object.Integer{Value: leftVal - rightVal}
	case token.ASTERISK:
		return &object.Integer{Value: leftVal * rightVal}
	case token.SLASH:
		return &object.Integer{Value: leftVal / rightVal}
	case token.EQ:
		return nativeBoolToBooleanObject(leftVal == rightVal)
	case token.NOT_EQ:
		return nativeBoolToBooleanObject(leftVal != rightVal)
	case token.GT:
		return nativeBoolToBooleanObject(leftVal > rightVal)
	case token.LT:
		return nativeBoolToBooleanObject(leftVal < rightVal)
	default:
		return newError("unknown operator:%s %s %s", left.Type(), operator, right.Type())
	}
}

// nativeBoolToBooleanObject  将原生的真假，转换为object
func nativeBoolToBooleanObject(value bool) *object.Boolean {
	if value {
		return TRUE
	}
	return FALSE
}

// evalPrefixExpression  eval prefix
func evalPrefixExpression(node *ast.PrefixExpression) object.Object {
	right := Eval(node.Right)
	if isError(right) {
		return right
	}
	switch node.Operator {
	case token.BANG:
		return evalBangOperatorExpression(right)
	case token.MINUS:
		return evalMinusOperatorExpression(right)
	default:
		return newError("unknown operator:%s%s", node.Operator, right.Type())
	}
}

// evalMinusOperatorExpression  eval minus
func evalMinusOperatorExpression(right object.Object) object.Object { // evalMinusOperatorExpression
	if right.Type() != object.INTEGER_OBJ {
		return newError("unknown operator:%s%s", token.MINUS, right.Type())
	}
	value := right.(*object.Integer).Value
	return &object.Integer{Value: -value}
}

// evalBangOperatorExpression  eval bang
func evalBangOperatorExpression(right object.Object) object.Object {
	switch right {
	case TRUE:
		return FALSE
	case FALSE:
		return TRUE
	case NULL:
		return TRUE
	default:
		return FALSE
	}
}

// evalStatements  eval statements
func evalStatements(statements []ast.Statement) object.Object {
	var result object.Object
	for _, statement := range statements {
		result = Eval(statement)
		//if current statement is returnStatement; jump other statements
		if returnValue, ok := result.(*object.Return); ok {
			return returnValue
		}
		if errorValue, ok := result.(*object.Error); ok {
			return errorValue
		}
	}
	return result
}
