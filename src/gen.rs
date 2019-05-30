use std::collections::HashMap;
use crate::parser::{Program, Stmt, Expr, Literal, Infix};

pub struct Generator {
    pub code: String,
    label_num: u32,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            code: String::new(),
            label_num: 0,
        }
    }

    fn gen_lvalue(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(offset) => {
                self.code.push_str("  mov rax, rbp\n");
                self.code.push_str(&format!("  sub rax, {}\n", offset));
                self.code.push_str("  push rax\n");
            },
            _ => {
                println!("代入の左辺値が変数ではありません");
            },
        };
    }

    fn gen_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(Literal::Number(num)) => {
                self.code.push_str(&format!("  push {}\n", num));
            },
            Expr::Ident(_) => {
                self.gen_lvalue(expr);
                self.code.push_str("  pop rax\n");
                self.code.push_str("  mov rax, [rax]\n");
                self.code.push_str("  push rax\n");
            },
            Expr::Assign(lhs, rhs) => {
                self.gen_lvalue(lhs);
                self.gen_expr(rhs);

                self.code.push_str("  pop rdi\n");
                self.code.push_str("  pop rax\n");
                self.code.push_str("  mov [rax], rdi\n");
                self.code.push_str("  push rdi\n");
            },
            Expr::Infix(kind, lhs, rhs) => {
                self.gen_expr(lhs);
                self.gen_expr(rhs);

                self.code.push_str("  pop rdi\n");
                self.code.push_str("  pop rax\n");

                self.code.push_str(match kind {
                    Infix::Add => "  add rax, rdi\n",
                    Infix::Sub => "  sub rax, rdi\n",
                    Infix::Mul => "  imul rdi\n",
                    Infix::Div => "  cqo\n  idiv rdi\n",
                    Infix::Equal => "  cmp rax, rdi\n  sete al\n  movzb  rax, al\n",
                    Infix::NotEqual => "  cmp rax, rdi\n  setne al\n  movzb  rax, al\n",
                    Infix::LessThan => "  cmp rax, rdi\n  setl al\n  movzb  rax, al\n",
                    Infix::LessThanOrEqual => "  cmp rax, rdi\n  setle al\n  movzb  rax, al\n",
                });

                self.code.push_str("  push rax\n");
            }
            _ => {},
        };
    }

    pub fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => self.gen_expr(&expr),
            Stmt::Return(expr) => {
                self.gen_expr(&expr);
                self.code.push_str("  pop rax\n");
                self.code.push_str("  mov rsp, rbp\n");
                self.code.push_str("  pop rbp\n");
                self.code.push_str("  ret\n");
            },
            Stmt::If(cond, if_stmt, else_stmt) => {
                self.gen_expr(&cond);
                self.code.push_str("  pop rax\n");
                self.code.push_str("  cmp rax, 0\n");
                self.label_num += 1;
                let label_num = self.label_num;

                // else 節がある場合
                if let Some(else_stmt) = else_stmt {
                    self.code.push_str(&format!("  je .Lelse{}\n", label_num));
                    self.gen_stmt(if_stmt);
                    self.code.push_str(&format!("  jmp .Lend{}\n", label_num));
                    self.code.push_str(&format!(".Lelse{}:\n", label_num));
                    self.gen_stmt(else_stmt);
                } else {
                    self.code.push_str(&format!("  je .Lend{}\n", label_num));
                    self.gen_stmt(if_stmt);
                }

                self.code.push_str(&format!(".Lend{}:\n", label_num));
            },
            _ => {},
        }
    }

    pub fn gen(&mut self, program: &Program, variables: &HashMap<String, usize>) {
        self.code.push_str(".intel_syntax noprefix\n");
        self.code.push_str(".global main\n");
        self.code.push_str("main:\n");

        self.code.push_str("  push rbp\n");
        self.code.push_str("  mov rbp, rsp\n");
        self.code.push_str(&format!("  sub rsp, {}\n", variables.len() * 8));

        for stmt in &program.0 {
            self.gen_stmt(&stmt);
            self.code.push_str("  pop rax\n");
        }
    }
}
