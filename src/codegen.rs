use crate::ast::*;
use std::fmt::Write;

pub struct CodeGenerator {
    buffer: String,
    indent: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            indent: 0,
        }
    }

    pub fn generate(mut self, program: &Program) -> String {
        for item in &program.items {
            self.generate_item(item);
            self.buffer.push('\n');
        }
        self.buffer
    }

    fn indent(&mut self) {
        for _ in 0..self.indent {
            self.buffer.push_str("    ");
        }
    }

    fn generate_item(&mut self, item: &Item) {
        match item {
            Item::Variable(var) => {
                self.indent();
                let vis = match var.visibility {
                    Visibility::Public => "public ",
                    Visibility::Private => "",
                    Visibility::Default => "",
                };
                let kind = match var.ty {
                    Type::List => "list",
                    _ => "var",
                };
                write!(self.buffer, "{} {} {} = ", vis, kind, var.name).unwrap();
                self.generate_expr(&var.init);
                self.buffer.push_str(";\n");
            }
            Item::Procedure(proc) => {
                self.generate_proc(proc);
            }
            Item::Function(func) => {
                self.generate_func(func);
            }
            Item::Costume(c) => {
                self.indent();
                write!(self.buffer, "costume \"{}\", \"{}\"", c.name, c.path).unwrap();
                if let Some(x) = c.x {
                    if let Some(y) = c.y {
                        write!(self.buffer, ", {}, {}", x, y).unwrap();
                    }
                }
                self.buffer.push_str(";\n");
            }
            Item::Sound(s) => {
                self.indent();
                write!(self.buffer, "sound \"{}\", \"{}\";\n", s.name, s.path).unwrap();
            }
            Item::Comment(c) => {
                self.indent();
                writeln!(self.buffer, "{}", c).unwrap();
            }
            Item::Stmt(stmt) => {
                self.indent();
                self.generate_stmt(stmt);
                self.buffer.push('\n');
            }
            Item::BatchBreak => {
                self.buffer.push('\n');
            }
            Item::Use(u) => {
                self.indent();
                writeln!(self.buffer, "use {};", u).unwrap();
            }
            _ => {
                self.indent();
                self.buffer.push_str("// Unknown item\n");
            }
        }
    }

    fn generate_proc(&mut self, proc: &ProcedureDef) {
        self.indent();
        if let Some(comment) = &proc.comment {
            writeln!(self.buffer, "{}", comment).unwrap();
            self.indent();
        }
        if proc.is_warp {
            self.buffer.push_str("#[warp]\n");
            self.indent();
        }
        write!(self.buffer, "proc {}(", proc.name).unwrap();
        for (i, param) in proc.params.iter().enumerate() {
            if i > 0 {
                self.buffer.push_str(", ");
            }
            write!(self.buffer, "{}: {}", param.name, type_str(&param.ty)).unwrap();
        }
        self.buffer.push(')');
        if let Some(rt) = &proc.return_type {
            write!(self.buffer, " -> {}", type_str(rt)).unwrap();
        }
        self.buffer.push_str(" {\n");
        self.indent += 1;
        for stmt in &proc.body {
            self.indent();
            self.generate_stmt(stmt);
            self.buffer.push('\n');
        }
        self.indent -= 1;
        self.indent();
        self.buffer.push_str("}\n");
    }

    fn generate_func(&mut self, func: &Function) {
        self.indent();
        if let Some(comment) = &func.comment {
            writeln!(self.buffer, "{}", comment).unwrap();
            self.indent();
        }
        for attr in &func.attributes {
            write!(self.buffer, "#[{}", attr.name).unwrap();
            if !attr.args.is_empty() {
                self.buffer.push('(');
                for (i, arg) in attr.args.iter().enumerate() {
                    if i > 0 {
                        self.buffer.push_str(", ");
                    }
                    self.generate_expr(arg);
                }
                self.buffer.push(')');
            }
            self.buffer.push_str("]\n");
            self.indent();
        }
        write!(self.buffer, "fn {}(", func.name).unwrap();
        // params
        self.buffer.push_str(") {\n");
        self.indent += 1;
        for stmt in &func.body {
            self.indent();
            self.generate_stmt(stmt);
            self.buffer.push('\n');
        }
        self.indent -= 1;
        self.indent();
        self.buffer.push_str("}\n");
    }

    fn generate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Assign(name, expr, _) => {
                write!(self.buffer, "{} = ", name).unwrap();
                self.generate_expr(expr);
                self.buffer.push(';');
            }
            Stmt::Expr(expr, _) => {
                self.generate_expr(expr);
                self.buffer.push(';');
            }
            Stmt::Let(name, expr, _) => {
                write!(self.buffer, "let {} = ", name).unwrap();
                self.generate_expr(expr);
                self.buffer.push(';');
            }
            Stmt::Return(expr, _) => {
                self.buffer.push_str("return");
                if let Some(e) = expr {
                    self.buffer.push(' ');
                    self.generate_expr(e);
                }
                self.buffer.push(';');
            }
            Stmt::If(cond, then_block, else_block, _) => {
                self.buffer.push_str("if ");
                self.generate_expr(cond);
                self.buffer.push_str(" {\n");
                self.indent += 1;
                for s in then_block {
                    self.indent();
                    self.generate_stmt(s);
                    self.buffer.push('\n');
                }
                self.indent -= 1;
                self.indent();
                self.buffer.push('}');
                if let Some(else_b) = else_block {
                    self.buffer.push_str(" else {\n");
                    self.indent += 1;
                    for s in else_b {
                        self.indent();
                        self.generate_stmt(s);
                        self.buffer.push('\n');
                    }
                    self.indent -= 1;
                    self.indent();
                    self.buffer.push('}');
                }
            }
            Stmt::Repeat(count, body, _) => {
                self.buffer.push_str("repeat(");
                self.generate_expr(count);
                self.buffer.push_str(") {\n");
                self.indent += 1;
                for s in body {
                    self.indent();
                    self.generate_stmt(s);
                    self.buffer.push('\n');
                }
                self.indent -= 1;
                self.indent();
                self.buffer.push('}');
            }
            Stmt::Forever(body, _) => {
                self.buffer.push_str("forever {\n");
                self.indent += 1;
                for s in body {
                    self.indent();
                    self.generate_stmt(s);
                    self.buffer.push('\n');
                }
                self.indent -= 1;
                self.indent();
                self.buffer.push('}');
            }
            Stmt::Until(cond, body, _) => {
                self.buffer.push_str("until ");
                self.generate_expr(cond);
                self.buffer.push_str(" {\n");
                self.indent += 1;
                for s in body {
                    self.indent();
                    self.generate_stmt(s);
                    self.buffer.push('\n');
                }
                self.indent -= 1;
                self.indent();
                self.buffer.push('}');
            }
            Stmt::Comment(c) => {
                write!(self.buffer, "{}", c).unwrap();
            }
            Stmt::Match(expr, arms, _, _) => {
                self.buffer.push_str("match ");
                self.generate_expr(expr);
                self.buffer.push_str(" {\n");
                self.indent += 1;
                for (pat, stmts) in arms {
                    self.indent();
                    self.generate_expr(pat);
                    self.buffer.push_str(" => {\n");
                    self.indent += 1;
                    for s in stmts {
                        self.indent();
                        self.generate_stmt(s);
                        self.buffer.push('\n');
                    }
                    self.indent -= 1;
                    self.indent();
                    self.buffer.push_str("}\n");
                }
                self.indent -= 1;
                self.indent();
                self.buffer.push('}');
            }
            Stmt::CBlock(name, args, body, _) => {
                write!(self.buffer, "{}(", name).unwrap();
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.buffer.push_str(", ");
                    }
                    self.generate_expr(arg);
                }
                self.buffer.push_str(") {\n");
                self.indent += 1;
                for s in body {
                    self.indent();
                    self.generate_stmt(s);
                    self.buffer.push('\n');
                }
                self.indent -= 1;
                self.indent();
                self.buffer.push('}');
            }
        }
    }

    fn generate_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(f) => write!(self.buffer, "{}", f).unwrap(),
            Expr::String(s) => write!(self.buffer, "{:?}", s).unwrap(),
            Expr::Bool(b) => write!(self.buffer, "{}", b).unwrap(),
            Expr::Variable(v) => write!(self.buffer, "{}", v).unwrap(),
            Expr::BinOp(l, op, r) => {
                self.generate_expr(l);
                let op_str = match op {
                    Op::Add => "+",
                    Op::Sub => "-",
                    Op::Mul => "*",
                    Op::Div => "/",
                    Op::Mod => "%",
                    Op::Eq => "==",
                    Op::Gt => ">",
                    Op::Lt => "<",
                    Op::And => "&&",
                    Op::Or => "||",
                    Op::Ne => "!=",
                    Op::Ge => ">=",
                    Op::Le => "<=",
                };
                write!(self.buffer, " {} ", op_str).unwrap();
                self.generate_expr(r);
            }
            Expr::Call(name, args) => {
                write!(self.buffer, "{}(", name).unwrap();
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.buffer.push_str(", ");
                    }
                    self.generate_expr(arg);
                }
                self.buffer.push(')');
            }
            Expr::List(items) => {
                self.buffer.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        self.buffer.push_str(", ");
                    }
                    self.generate_expr(item);
                }
                self.buffer.push(']');
            }
            _ => {
                self.buffer.push_str("/* unknown expr */");
            }
        }
    }
}

fn type_str(t: &Type) -> &'static str {
    match t {
        Type::Number => "number",
        Type::String => "string",
        Type::Boolean => "boolean",
        Type::List => "list",
        Type::Unknown => "unknown",
    }
}
