use crate::token::*;
use crate::error::{CompileError, Span};
use crate::id::IdMap;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    pub tokens: Vec<Token>,
    pub errors: Vec<CompileError>,
    id_map: &'a mut IdMap,
    input: Vec<char>,
    pos: usize,
    ch: char,
    line: usize,
    col: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str, id_map: &'a mut IdMap) -> Self {
        Tokenizer {
            tokens: Vec::new(),
            input: input.chars().collect(),
            id_map,
            pos: 0,
            ch: input.chars().next().unwrap_or('\0'),
            errors: Vec::new(),
            line: 0,
            col: 0,
        }
    }

    fn next(&mut self) {
        self.pos += 1;
        self.col += 1;
        self.ch = match self.input.get(self.pos) {
            Some(ch) => *ch, 
            None => '\0',
        };
    }

    fn add_error(&mut self, msg: &str) {
        self.errors.push(CompileError {
            span: Span::new(self.line, self.col, self.line, self.col),
            msg: msg.to_string(),
        });
    }

    fn add_token(&mut self, kind: TokenKind, start_col: usize, end_col: usize) {
        self.tokens.push(Token {
            kind,
            start_line: self.line,
            end_line: self.line,
            start_col,
            end_col,
        });
    }

    fn add_token_and_skip(&mut self, kind: TokenKind, skip: usize) {
        self.add_token(kind, self.col, self.col + skip);
        for _ in 0..skip {
            self.next();
        }
    }

    fn next_is(&self, ch: char) -> bool {
        match self.input.get(self.pos + 1) {
            Some(next) => ch == *next,
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                ' ' | '\t' | '\r' => self.next(),
                _ => break,
            }
        }
    }

    fn tokenize_number(&mut self, radix: u32) {
        let start_col = self.col;
        let mut num: u32 = 0;
        while self.ch.is_digit(radix) {
            num = (radix * num) + self.ch.to_digit(radix).unwrap() as u32;
            self.next();
        }

        self.add_token(TokenKind::Number(num as i64), start_col, self.col);
    }

    fn tokenize_decimal(&mut self) {
        let start_col = self.col;
        let mut num: i64 = 0;
        let mut decimal_point_pos: Option<u32> = None;
        while self.ch.is_digit(10) || self.ch == '.' {
            if self.ch == '.' {
                decimal_point_pos = Some((self.col - start_col) as u32);
            } else {
                let digit = self.ch.to_digit(10).unwrap() as u8;
                num = (10 * num) + digit as i64;
            }

            self.next();
        }

        if let Some(pos) = decimal_point_pos {
            let pos = (self.col - start_col) as u32 - pos;

            if self.ch == 'f' || self.ch == 'F' {
                self.add_token(TokenKind::FloatNum(num as f32 / 10u32.pow(pos - 1) as f32), start_col, self.col);
                self.next();
            } else {
                self.add_token(TokenKind::DoubleNum(num as f64 / 10u32.pow(pos - 1) as f64), start_col, self.col);
            }
        } else {
            self.add_token(TokenKind::Number(num as i64), start_col, self.col);
        }
    }

    fn tokenize_ident(&mut self) {
        let start_pos = self.pos;
        let start_col = self.col;

        loop {
            match self.ch {
                c if c.is_ascii_alphanumeric() || c == '_' => self.next(),
                _ => break,
            }
        }

        let s: String = self.input[start_pos..self.pos].iter().collect();
        let token = match &*s {
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "int" => TokenKind::Int,
            "sizeof" => TokenKind::SizeOf,
            "char" => TokenKind::Char,
            "struct" => TokenKind::Struct,
            "void" => TokenKind::Void,
            "typedef" => TokenKind::Typedef,
            "short" => TokenKind::Short,
            "long" => TokenKind::Long,
            "enum" => TokenKind::Enum,
            "static" => TokenKind::Static,
            "switch" => TokenKind::Switch,
            "case" => TokenKind::Case,
            "break" => TokenKind::Break,
            "extern" => TokenKind::Extern,
            "continue" => TokenKind::Continue,
            "default" => TokenKind::Default,
            "const" => TokenKind::Const,
            "float" => TokenKind::Float,
            "double" => TokenKind::Double,
            "goto" => TokenKind::Goto,
            s => {
                let id = self.id_map.new_id(&s);
                TokenKind::Ident(id)
            }
        };
        self.add_token(token, start_col, self.col);
    }

    pub fn tokenize_string(&mut self) {
        // " を消費
        self.next();

        let start_pos = self.pos;
        let start_col = self.col - 1;
        loop {
            match self.ch {
                '"' => {
                    self.next();
                    break;
                },
                '\0' | '\n' => {
                    self.add_error("対応する \" がありません");
                    return;
                },
                _ => self.next(),
            };
        }

        let s: String = self.input[start_pos..self.pos - 1].iter().collect();
        let id = self.id_map.new_id(&s);
        self.add_token(TokenKind::String(id), start_col, self.col);
    }

    pub fn skip_single_line_comment(&mut self) {
        // 改行が来るまでスキップ
        loop {
            match self.ch {
                '\n' => break,
                _ => self.next(),
            };
        }
    }

    pub fn skip_block_comment(&mut self) {
        // "*/" が来るまでスキップ
        loop {
            match self.ch {
                '*' if self.next_is('/') => {
                    self.next();
                    self.next();
                    break;
                },
                '\n' => {
                    self.next();
                    self.line += 1;
                    self.col = 0;
                },
                _ => self.next(),
            };
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, Vec<CompileError>> {
        loop {
            self.skip_whitespace();

            match self.ch {
                '0' if self.next_is('x') => {
                    // "0x" を消費
                    self.next();
                    self.next();
                    self.tokenize_number(16)
                },
                '0' if self.next_is('.') => self.tokenize_decimal(),
                '0' => {
                    // "0" を消費
                    self.next();
                    self.tokenize_number(8);
                },
                c if c.is_digit(10) => self.tokenize_decimal(),
                c if c.is_ascii_alphanumeric() || c == '_' => self.tokenize_ident(),
                '+' => self.add_token_and_skip(TokenKind::Add, 1),
                '-' if self.next_is('>') => self.add_token_and_skip(TokenKind::Arrow, 2),
                '-' => self.add_token_and_skip(TokenKind::Sub, 1),
                '*' => self.add_token_and_skip(TokenKind::Asterisk, 1),
                '/' if self.next_is('/') => self.skip_single_line_comment(),
                '/' if self.next_is('*') => self.skip_block_comment(),
                '/' => self.add_token_and_skip(TokenKind::Div, 1),
                '(' => self.add_token_and_skip(TokenKind::Lparen, 1),
                ')' => self.add_token_and_skip(TokenKind::Rparen, 1),
                '{' => self.add_token_and_skip(TokenKind::Lbrace, 1),
                '}' => self.add_token_and_skip(TokenKind::Rbrace, 1),
                '[' => self.add_token_and_skip(TokenKind::Lbracket, 1),
                ']' => self.add_token_and_skip(TokenKind::Rbracket, 1),
                '=' if self.next_is('=') => self.add_token_and_skip(TokenKind::Equal, 2),
                '=' => self.add_token_and_skip(TokenKind::Assign, 1),
                '!' if self.next_is('=') => self.add_token_and_skip(TokenKind::NotEqual, 2),
                '<' if self.next_is('<') => self.add_token_and_skip(TokenKind::Shl, 2),
                '<' if self.next_is('=') => self.add_token_and_skip(TokenKind::LessThanOrEqual, 2),
                '<' => self.add_token_and_skip(TokenKind::LessThan, 1),
                '>' if self.next_is('>') => self.add_token_and_skip(TokenKind::Shr, 2),
                '>' if self.next_is('=') => self.add_token_and_skip(TokenKind::GreaterThanOrEqual, 2),
                '>' => self.add_token_and_skip(TokenKind::GreaterThan, 1),
                ';' => self.add_token_and_skip(TokenKind::Semicolon, 1),
                ',' => self.add_token_and_skip(TokenKind::Comma, 1),
                '&' => self.add_token_and_skip(TokenKind::Ampersand, 1),
                '"' => self.tokenize_string(),
                '|' => self.add_token_and_skip(TokenKind::Or, 1),
                '^' => self.add_token_and_skip(TokenKind::Xor, 1),
                '~' => self.add_token_and_skip(TokenKind::BitNot, 1),
                '%' => self.add_token_and_skip(TokenKind::Mod, 1),
                '#' => self.add_token_and_skip(TokenKind::Hash, 1),
                '.' => match self.input.get(self.pos + 1) {
                    Some(ch) if ch.is_digit(10) => self.tokenize_decimal(),
                    _ => self.add_token_and_skip(TokenKind::Dot, 1),
                },
                ':' => self.add_token_and_skip(TokenKind::Colon, 1),
                '\\' if self.next_is('\n') => {
                    self.next();
                    self.next();
                    self.line += 1;
                    self.col = 0;
                },
                '\n' => {
                    self.add_token_and_skip(TokenKind::NewLine, 1);
                    self.line += 1;
                    self.col = 0;
                },
                '\0' => break,
                _ => { self.add_error("Unexpected token"); self.next() },
            }
        }

        self.add_token(TokenKind::EOF, self.col, self.col);

        if self.errors.len() > 0 {
            Err(self.errors)
        } else {
            Ok(self.tokens)
        }
    }
}
