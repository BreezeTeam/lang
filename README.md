# lang

An interpreter for the programming language written in Rust and Golang

## 解释器的结构:

0. repl循环(Read-Eval-Print Loop)
   > 交互式编程环境的顶层构建
   > 循环用于读取用户输入的字符流，分行读取，输入至解释器中，完成整个语法解析和执行和过程后输出结果.
1. 词法分析(lexer analysis)
   > 词法分析器读入组成源程序的字符流，并且将其组织为词素序列，每个词素形式如`<token_name,attribute_value>`。
   > 词法分析器在编译器中负责读取源程序，因此他还会完成一些识别词素之外的其他任务
   > 1.过滤掉源程序中的注释和空白(空格，换行符，制表符，以及在输入中用于分割词法单元的其他字符)
   > 2.将编译器生成的错误与源程序的位置联系起来
2. 语法分析(parser analysis)并得到抽象语法树(ast)
   > 通过 Pratt Parsing 进行语法分析
   这是一种自上而下操作符优先级解析方法，是作为基于上下文无关文化和Backus-Naur-From 解析器的替代方案
   > 当完成 语法分析后，我们会判断该语言的基本语法问题，并且构造一个ast树，该树由表达式(Expression)和语句(Statement)组成
   > 树根即为 Program ast root node，它由语句列表组成
4. 对象系统(object)
   > 在开始构建 我们 eval ast 解释执行器之前，我们首先需要定义我们的对象系统
   > 该系统将解释的值对象和解释器编写语言（golang）的对象进行关联，并且通过定义合适的接口，来向使用语言的用户提供数据
   > 简单的说，语义分析中需要对ast节点进行计算，我们会将计算结果存储在我们定义的对象系统中
   > 例如，我们定义一个 integer 对象，当语法分析遇到 数字时，我们会将其转换为 ast 对象，在然后的语义分析中，deep_walking 对
   ast 进行计算
   > 将计算结果封装到 integer 对象中，并返回其引用
   > 该语言主要有 int，bool，function，string，builtin，array，hash(for `{"name": "Monkey"}`),null,return,error(for eval
   statements errors)
4. 语义分析(eval)
   > 通过对构建得到的ast进行 tree-walking 实现语义分析，并进行解释执行
   > 通过构建一个eval 函数进行递归计算表达式，表达式的具体计算在go中实现

## 附

编译器:
是一种将源程序翻译为另一种目标语言编写的程序

解释器:
直接将用户输入的源程序，执行其中的操作并输出

虚拟机:
源程序=>翻译器=>中间程序=>虚拟机
将中间程序和输入一起交给虚拟机执行并输出结果

语言处理器:
源程序=>预处理器=>预处理后的源程序=>编译器=>目标汇编程序=>汇编器=>可重定位机器代码=>连接器/加载器=>目标机器代码

