use conch_parser::ast::{
    AndOrList, Arithmetic, Command, ComplexWord, CompoundCommand, CompoundCommandKind,
    ListableCommand, Parameter, ParameterSubstitution, PipeableCommand, Redirect,
    RedirectOrCmdWord, SimpleCommand, SimpleWord, TopLevelCommand, TopLevelWord, Word,
};
use conch_parser::lexer::Lexer;
use conch_parser::parse::DefaultParser;
use std::rc::Rc;

fn main() {
    // Initialize our token lexer and shell parser with the program's input
    let cmd = r#"
DEBUG=true echo foo ${BAR}
if []
    "#;
    let lex = Lexer::new(cmd.chars());
    let parser = DefaultParser::new(lex);

    // Parse our input!
    for t in parser {
        match t {
            Err(e) => {
                eprintln!("ERROR");
                eprintln!("{:?}", e);
                eprintln!("{}", e.to_string());
            }
            Ok(TopLevelCommand(tlc)) => match tlc {
                Command::Job(_) => {}
                Command::List(list) => match list {
                    AndOrList { first, rest } => {
                        println!("rest={:?}", rest);
                        match first {
                            ListableCommand::Pipe(_, _) => {}
                            ListableCommand::Single(single) => match single {
                                PipeableCommand::Simple(simple) => {
                                    for w in simple.redirects_or_cmd_words {
                                        match w {
                                            RedirectOrCmdWord::Redirect(_) => {}
                                            RedirectOrCmdWord::CmdWord(cmd_word) => {
                                                match cmd_word {
                                                    TopLevelWord(tlw) => {
                                                        println!("tlw={:?}", tlw);
                                                        match tlw {
                                                            ComplexWord::Concat(_) => {}
                                                            ComplexWord::Single(single) => match single {
                                                                Word::Simple(w_s) => match w_s {
                                                                    SimpleWord::Literal(_) => {}
                                                                    SimpleWord::Escaped(_) => {}
                                                                    SimpleWord::Param(p) => match p {
                                                                        Parameter::At => {}
                                                                        Parameter::Star => {}
                                                                        Parameter::Pound => {}
                                                                        Parameter::Question => {}
                                                                        Parameter::Dash => {}
                                                                        Parameter::Dollar => {}
                                                                        Parameter::Bang => {}
                                                                        Parameter::Positional(_) => {}
                                                                        Parameter::Var(var) => {
                                                                            println!("var={}", var);
                                                                        }
                                                                    },
                                                                    SimpleWord::Subst(_) => {}
                                                                    SimpleWord::Star => {}
                                                                    SimpleWord::Question => {}
                                                                    SimpleWord::SquareOpen => {}
                                                                    SimpleWord::SquareClose => {}
                                                                    SimpleWord::Tilde => {}
                                                                    SimpleWord::Colon => {}
                                                                },
                                                                Word::DoubleQuoted(_) => {}
                                                                Word::SingleQuoted(_) => {}
                                                            },
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                PipeableCommand::Compound(_) => {}
                                PipeableCommand::FunctionDef(_, _) => {}
                            },
                        }
                    }
                },
            },
        }
    }
}
