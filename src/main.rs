#![feature(box_patterns, bind_by_move_pattern_guards)]

mod tokenizer;
mod parser;
mod gen;
mod sema;
mod ast;
mod error;
mod token;
mod preprocess;
mod id;

use std::env;
use std::process;
use std::fs::File;
use std::io::Read;
use tokenizer::Tokenizer;
use parser::Parser;
use gen::Generator;
use error::CompileError;
use preprocess::preprocess;
use id::IdMap;
use colored::*;

enum ShowType {
    Token,
    Program,
    Preprocess,
    Code,
}

fn print_error(lines: &Vec<&str>, errors: &Vec<CompileError>) {
    for error in errors {
        let line = (error.span.start_line + 1).to_string();
        println!("{} {}", format!("{} |", line).bright_cyan(), lines[error.span.start_line]);
        println!("{}{} {}", " ".repeat(error.span.start_col + line.chars().count() + 3).bright_red(), "^".repeat(error.span.end_col - error.span.start_col).bright_red(), error.msg.bright_red());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("引数の個数が正しくありません");
        process::exit(1);
    }

    let filepath = &args[1];
    let mut file = File::open(filepath).expect("Unable to open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Unable to read file");

    let lines: Vec<&str> = input.split('\n').collect();

    let show_type = match args.get(2) {
        Some(s) => match &s[..] {
            "token" => ShowType::Token,
            "program" => ShowType::Program,
            "preprocess" => ShowType::Preprocess,
            _ => ShowType::Code,
        },
        _ => ShowType::Code,
    };

    let mut id_map = IdMap::new();

    // 字句解析
    let tokenizer = Tokenizer::new(&input, &mut id_map);
    let tokens = match tokenizer.tokenize() {
        Ok(tokens) => tokens,
        Err(errors) => {
            print_error(&lines, &errors);
            process::exit(1);
        },
    };

    if let ShowType::Token = show_type {
        for token in tokens {
            println!("{:?}", token);
        }
        process::exit(1);
    }

    // プリプロセス
    let tokens = match preprocess(tokens, &mut id_map) {
        Ok(tokens) => tokens,
        Err(errors) => {
            print_error(&lines, &errors);
            process::exit(1);
        },
    };

    if let ShowType::Preprocess = show_type {
        for token in tokens {
            println!("{:?}", token);
        }
        process::exit(1);
    }

    // 構文解析
    let parser = Parser::new(tokens, &id_map);
    let mut program = match parser.parse() {
        Ok(program) => program,
        Err(errors) => {
            print_error(&lines, &errors);
            process::exit(1);
        }
    };

    if let ShowType::Program = show_type {
        dbg!(program);
        process::exit(1);
    }

    // 意味解析
    if let Err(errors) = sema::walk(&mut program) {
        print_error(&lines, &errors);
        process::exit(1);
    }

    // コード生成
    let mut generator = Generator::new(id_map);
    generator.gen(program);

    if let ShowType::Code = show_type {
        println!("{}", generator.code);
    }
}
