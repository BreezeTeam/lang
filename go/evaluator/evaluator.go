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
func Eval(node ast.Node, env *object.Environment) object.Object {
	switch node := node.(type) {
	case *ast.Program:
		result := evalStatements(node.Statement, env)
		if returnValue, ok := result.(*object.Return); ok {
			return returnValue.Value
		}
		return result
	case *ast.ExpressionStatement:
		return Eval(node.Expression, env)
	case *ast.Identifier:
		return evalIdentifier(node, env)
	case *ast.IntegerLiteral:
		return &object.Integer{Value: node.Value}
	case *ast.Boolean:
		return nativeBoolToBooleanObject(node.Value)
	case *ast.FunctionLiteral:
		return &object.Function{Parameters: node.Parameters, Body: node.Body, Env: env}
	case *ast.StringLiteral:
		return &object.String{Value: node.Value}
	case *ast.ArrayLiteral:
		elements := evalExpression(node.Elements, env)
		if len(elements) == 1 && isError(elements[0]) {
			return elements[0]
		}
		return &object.Array{Elements: elements}
	case *ast.PrefixExpression:
		return evalPrefixExpression(node, env)
	case *ast.InfixExpression:
		return evalInfixExpression(node, env)
	case *ast.IfExpression:
		return evalIfExpression(node, env)
	case *ast.BlockStatement:
		return evalStatements(node.Statement, env)
	case *ast.ReturnStatement:
		return &object.Return{Value: Eval(node.ReturnValue, env)}
	case *ast.IndexExpression:
		//先得到array
		left := Eval(node.Left, env)
		if isError(left) {
			return left
		}
		// 计算索引
		index := Eval(node.Index, env)
		if isError(index) {
			return index
		}
		return evalIndexExpression(left, index)
	case *ast.CallExpression:
		//先得到 function 对象
		function := Eval(node.Function, env)
		if isError(function) {
			return function
		}
		args := evalExpression(node.Arguments, env)
		//如果只有参数解析发生错误
		if len(args) == 1 && isError(args[0]) {
			return args[0]
		}
		return applyFunction(function, args)
	case *ast.LetStatement:
		val := Eval(node.Value, env)
		if isError(val) {
			return val
		}
		env.Set(node.Name.Value, val)
	}

	return nil
}

// evalIndexExpression 根据 右值 从左值中获取对应位置的元素
func evalIndexExpression(left object.Object, index object.Object) object.Object {
	switch {
	case left.Type() == object.ARRAY_OBJ && index.Type() == object.INTEGER_OBJ:
		return evalArrayIndexExpression(left, index)
	default:
		return newError("index operator not supported: %s", left.Type())
	}
}

// evalArrayIndexExpression 将左值转为array 右值转为int，并且获取对应的元素
func evalArrayIndexExpression(left object.Object, index object.Object) object.Object {
	arrayObj := left.(*object.Array)
	idx := index.(*object.Integer)
	max := int64(len(arrayObj.Elements) - 1)
	if idx.Value < 0 || idx.Value > max {
		return NULL
	}
	return arrayObj.Elements[idx.Value]
}

// applyFunction 通过args传递给function，实现函数调用
func applyFunction(function object.Object, args []object.Object) object.Object {
	switch fn := function.(type) {
	case *object.Function:
		//拷贝一个 环境，这样就可以拷贝调用时的环境了，顺便把
		extendEnv := extendFunctionENV(fn, args)
		evaluated := Eval(fn.Body, extendEnv)
		return unwrapReturnValue(evaluated)
	case *object.Builtin:
		//内置函数
		return fn.Fn(args...)
	default:
		return newError("not a function:%s", function.Type())
	}
}

// unwrapReturnValue 将函数调用结果 解开，并返回结果;如果不是return，则直接返回最后的结果
func unwrapReturnValue(evaluated object.Object) object.Object {
	if returnValue, ok := evaluated.(*object.Return); ok {
		return returnValue.Value
	}
	return evaluated
}

func extendFunctionENV(function *object.Function, args []object.Object) *object.Environment {
	env := object.NewEnclosedEnvironment(function.Env)
	for idx, param := range function.Parameters {
		env.Set(param.Value, args[idx])
	}
	return env
}

// evalExpression 将每个 调用时的每个入参都执行一下，传到函数的env中
func evalExpression(exps []ast.Expression, env *object.Environment) []object.Object {
	var result []object.Object
	for _, e := range exps {
		evaluated := Eval(e, env)
		if isError(evaluated) {
			return []object.Object{evaluated}
		}
		result = append(result, evaluated)
	}
	return result
}

func evalIdentifier(node *ast.Identifier, env *object.Environment) object.Object {
	if val, ok := env.Get(node.Value); ok {
		return val
	}
	if builtin, ok := builtins[node.Value]; ok {
		return builtin
	}

	return newError("identifier not found: " + node.Value)
}

// evalIfExpression  eval if语句
func evalIfExpression(node *ast.IfExpression, env *object.Environment) object.Object {
	condition := Eval(node.Condition, env)
	if isError(condition) {
		return condition
	} else if isRealTrue(condition) {
		return Eval(node.Consequence, env)
	} else if node.Alternative != nil {
		return Eval(node.Alternative, env)
	} else {
		return NULL
	}
}

// evalInfixExpression  eval infix
func evalInfixExpression(node *ast.InfixExpression, env *object.Environment) object.Object {
	left := Eval(node.Left, env)
	if isError(left) {
		return left
	}
	right := Eval(node.Right, env)
	if isError(right) {
		return right
	}
	switch {
	case left.Type() == object.INTEGER_OBJ && right.Type() == object.INTEGER_OBJ:
		return evalIntegerInfixExpression(node.Operator, left, right)
	case left.Type() == object.STRING_OBJ && right.Type() == object.STRING_OBJ:
		return evalStringInfixExpression(node.Operator, left, right)
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

// evalStringInfixExpression eval string
func evalStringInfixExpression(operator string, left object.Object, right object.Object) object.Object {
	leftVal := left.(*object.String).Value
	rightVal := right.(*object.String).Value
	switch operator {
	case token.PLUS:
		return &object.String{Value: leftVal + rightVal}
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
func evalPrefixExpression(node *ast.PrefixExpression, env *object.Environment) object.Object {
	right := Eval(node.Right, env)
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
func evalStatements(statements []ast.Statement, env *object.Environment) object.Object {
	var result object.Object
	for _, statement := range statements {
		result = Eval(statement, env)
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
