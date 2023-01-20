extern crate lang_lib;
extern crate nom;
extern crate rustyline;
extern crate rustyline_derive;

use lang_lib::evaluator::*;
use lang_lib::lexer::*;
use lang_lib::parser::*;
use lang_lib::token::*;
use nom::Err;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
// use rustyline::config::OutputStreamType;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{Cmd, CompletionType, Config, Context, EditMode, Editor, KeyEvent};
use rustyline_derive::Helper;
use std::borrow::Cow::{self, Borrowed, Owned};

#[derive(Helper)]
struct MyHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Completer for MyHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for MyHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for MyHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

fn main() -> rustyline::Result<()> {
    let config = Config::builder()
        .history_ignore_space(true) // 保存以空格开始的行
        .completion_type(CompletionType::List) // 显示匹配列表
        .edit_mode(EditMode::Emacs) //编辑模式
        .build();
    let h = MyHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(h)); // 注册一个helper
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    println!();
    println!("This is the language repl");
    println!("Press Ctrl-D or enter \"quit\" to exit.");
    println!();

    let mut evaluator = Evaluator::new();
    let mut count = 1;

    loop {
        let p = format!("{}> ", count);
        rl.helper_mut().expect("No helper").colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);
        // 读取一行，并显示一个提示符
        let readline = rl.readline(&p);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let lex_tokens = Lexer::lexing(line.as_bytes());
                match lex_tokens {
                    Ok((_, r)) => {
                        println!("{:?}", r);
                        let parsed = Parser::parsing(Tokens::new(&r));
                        match parsed {
                            Ok((_, program)) => {
                                let eval = evaluator.evaluation(program);
                                println!("{:?}", &program);
                                // println!("{:?}", &eval);
                            }
                            Err(Err::Error(_)) => println!("Parser error"),
                            Err(Err::Failure(_)) => println!("Parser failure"),
                            Err(Err::Incomplete(_)) => println!("Incomplete parsing"),
                        }
                    }
                    Err(Err::Error(_)) => println!("Lexer error"),
                    Err(Err::Failure(_)) => println!("Lexer failure"),
                    Err(Err::Incomplete(_)) => println!("Incomplete lexing"),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        count += 1;
    }

    rl.append_history("history.txt")
}
