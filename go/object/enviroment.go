package object

type Environment struct {
	store map[string]Object
	outer *Environment
}

// NewEnclosedEnvironment 创建一个闭包环境
func NewEnclosedEnvironment(outer *Environment) *Environment {
	env := NewEnvironment()
	env.outer = outer
	return env
}

// NewEnvironment 新建一个环境
func NewEnvironment() *Environment {
	return &Environment{store: make(map[string]Object), outer: nil}
}

func (e *Environment) Get(name string) (Object, bool) {
	obj, ok := e.store[name]
	if !ok && e.outer != nil {
		obj, ok = e.outer.Get(name)
	}
	return obj, ok
}

func (e *Environment) Set(name string, val Object) Object {
	e.store[name] = val
	return val
}
