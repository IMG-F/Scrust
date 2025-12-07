use crate::ast::*;
use std::collections::HashMap;

pub fn transform_program(program: &mut Program) {
    if !program_uses_let(program) {
        return;
    }

    // 1. Inject Memory System Globals and Helpers
    inject_memory_system(program);

    // 2. Transform Procedures and Functions (Scripts)
    let mut new_items = Vec::new();
    let items = std::mem::take(&mut program.items);

    for item in items {
        match item {
            Item::Procedure(def) => {
                if block_uses_let(&def.body) {
                    let (wrapper, inner) = split_procedure(def);
                    new_items.push(Item::Procedure(wrapper));
                    new_items.push(Item::Procedure(inner));
                } else {
                    new_items.push(Item::Procedure(def));
                }
            }
            Item::Function(func) => {
                if block_uses_let(&func.body) {
                    let (wrapper_stmts, inner_proc) = split_function(func.clone());
                    let mut new_func = func;
                    new_func.body = wrapper_stmts;
                    new_items.push(Item::Function(new_func));
                    new_items.push(Item::Procedure(inner_proc));
                } else {
                    new_items.push(Item::Function(func));
                }
            }
            _ => new_items.push(item),
        }
    }

    program.items = new_items;
}

fn program_uses_let(program: &Program) -> bool {
    for item in &program.items {
        match item {
            Item::Procedure(def) => {
                if block_uses_let(&def.body) {
                    return true;
                }
            }
            Item::Function(func) => {
                if block_uses_let(&func.body) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn block_uses_let(stmts: &[Stmt]) -> bool {
    for stmt in stmts {
        match stmt {
            Stmt::Let(_, _, _) => return true,
            Stmt::If(_, t, e, _) => {
                if block_uses_let(t) {
                    return true;
                }
                if let Some(e) = e {
                    if block_uses_let(e) {
                        return true;
                    }
                }
            }
            Stmt::Repeat(_, b, _) => {
                if block_uses_let(b) {
                    return true;
                }
            }
            Stmt::Forever(b, _) => {
                if block_uses_let(b) {
                    return true;
                }
            }
            Stmt::Until(_, b, _) => {
                if block_uses_let(b) {
                    return true;
                }
            }
            Stmt::Match(_, cases, def, _) => {
                for (_, b) in cases {
                    if block_uses_let(b) {
                        return true;
                    }
                }
                if let Some(d) = def {
                    if block_uses_let(d) {
                        return true;
                    }
                }
            }
            Stmt::CBlock(_, _, b, _) => {
                if block_uses_let(b) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn inject_memory_system(program: &mut Program) {
    let globals = vec![
        Item::Variable(VariableDecl {
            name: "_RAM".to_string(),
            ty: Type::List,
            init: Expr::List(vec![]),
            visibility: Visibility::Public,
            comment: None,
        }),
        Item::Variable(VariableDecl {
            name: "_FREE_PAGES".to_string(),
            ty: Type::List,
            init: Expr::List(vec![]),
            visibility: Visibility::Public,
            comment: None,
        }),
        Item::Variable(VariableDecl {
            name: "_HIGH_WATER".to_string(),
            ty: Type::Number,
            init: Expr::Number(1.0),
            visibility: Visibility::Public,
            comment: None,
        }),
        Item::Variable(VariableDecl {
            name: "_RET_VAL".to_string(),
            ty: Type::Number,
            init: Expr::Number(0.0),
            visibility: Visibility::Public,
            comment: None,
        }),
    ];

    let sys_alloc = Item::Procedure(ProcedureDef {
        name: "sys_alloc".to_string(),
        params: vec![],
        return_type: None,
        is_warp: true,
        format: None,
        comment: None,
        body: parse_stmts(
            r#"
            if (length_of_list(_FREE_PAGES) > 0) {
                _RET_VAL = item_of_list(_FREE_PAGES, length_of_list(_FREE_PAGES));
                delete_of_list(_FREE_PAGES, length_of_list(_FREE_PAGES));
            } else {
                _RET_VAL = _HIGH_WATER;
                _HIGH_WATER += 16;
                repeat(16) {
                    add_to_list(_RAM, 0);
                }
            }
        "#,
        ),
    });

    let sys_free = Item::Procedure(ProcedureDef {
        name: "sys_free".to_string(),
        params: vec![Param {
            name: "ptr".to_string(),
            ty: Type::Number,
        }],
        return_type: None,
        is_warp: true,
        format: None,
        comment: None,
        body: parse_stmts(
            r#"
            add_to_list(_FREE_PAGES, ptr);
        "#,
        ),
    });

    let stack_set = Item::Procedure(ProcedureDef {
        name: "stack_set".to_string(),
        params: vec![
            Param {
                name: "ptr".to_string(),
                ty: Type::Number,
            },
            Param {
                name: "offset".to_string(),
                ty: Type::Number,
            },
            Param {
                name: "val".to_string(),
                ty: Type::Number,
            },
        ],
        return_type: None,
        is_warp: true,
        format: None,
        comment: None,
        body: parse_stmts(
            r#"
            replace_item_of_list(_RAM, ptr + offset, val);
        "#,
        ),
    });

    program.items.insert(0, stack_set);
    program.items.insert(0, sys_free);
    program.items.insert(0, sys_alloc);
    for g in globals {
        program.items.insert(0, g);
    }
}

fn parse_stmts(src: &str) -> Vec<Stmt> {
    let src = format!("{{ {} }}", src);
    match crate::parser::parse_block_only(&src) {
        Ok(stmts) => stmts,
        Err(_) => vec![],
    }
}

fn split_procedure(mut def: ProcedureDef) -> (ProcedureDef, ProcedureDef) {
    let inner_name = format!("_inner_{}", def.name);
    let base_param_name = "_base".to_string();

    let mut wrapper_body = Vec::new();
    wrapper_body.push(call_stmt("sys_alloc", vec![]));

    let mut inner_args: Vec<Expr> = def
        .params
        .iter()
        .map(|p| Expr::Variable(p.name.clone()))
        .collect();
    inner_args.push(Expr::Variable("_RET_VAL".to_string()));

    wrapper_body.push(call_stmt(&inner_name, inner_args));

    let wrapper = ProcedureDef {
        name: def.name.clone(),
        params: def.params.clone(),
        body: wrapper_body,
        return_type: def.return_type.clone(),
        is_warp: def.is_warp,
        format: def.format.clone(),
        comment: def.comment.clone(),
    };

    def.name = inner_name;
    def.params.push(Param {
        name: base_param_name.clone(),
        ty: Type::Number,
    });

    let mut transformer = BodyTransformer::new(base_param_name.clone());
    def.body = transformer.transform(def.body);

    def.body
        .push(call_stmt("sys_free", vec![Expr::Variable(base_param_name)]));

    (wrapper, def)
}

fn split_function(func: Function) -> (Vec<Stmt>, ProcedureDef) {
    let inner_name = format!("_inner_script_{}", uuid::Uuid::new_v4().simple());
    let base_param_name = "_base".to_string();

    let mut wrapper_body = Vec::new();
    wrapper_body.push(call_stmt("sys_alloc", vec![]));
    wrapper_body.push(call_stmt(
        &inner_name,
        vec![Expr::Variable("_RET_VAL".to_string())],
    ));

    let mut inner_def = ProcedureDef {
        name: inner_name,
        params: vec![Param {
            name: base_param_name.clone(),
            ty: Type::Number,
        }],
        body: func.body,
        return_type: None,
        is_warp: func.attributes.iter().any(|a| a.name == "warp"),
        format: None,
        comment: None,
    };

    let mut transformer = BodyTransformer::new(base_param_name.clone());
    inner_def.body = transformer.transform(inner_def.body);

    inner_def
        .body
        .push(call_stmt("sys_free", vec![Expr::Variable(base_param_name)]));

    (wrapper_body, inner_def)
}

fn call_stmt(name: &str, args: Vec<Expr>) -> Stmt {
    Stmt::Expr(Expr::Call(name.to_string(), args), None)
}

struct BodyTransformer {
    base_var: String,
    scope_offset: usize,
    var_map: HashMap<String, usize>,
}

impl BodyTransformer {
    fn new(base_var: String) -> Self {
        Self {
            base_var,
            scope_offset: 0,
            var_map: HashMap::new(),
        }
    }

    fn transform_scoped(&mut self, stmts: Vec<Stmt>) -> Vec<Stmt> {
        let saved_map = self.var_map.clone();
        let res = self.transform(stmts);
        self.var_map = saved_map;
        res
    }

    fn transform(&mut self, stmts: Vec<Stmt>) -> Vec<Stmt> {
        let mut new_stmts = Vec::new();
        for stmt in stmts {
            new_stmts.extend(self.transform_stmt(stmt));
        }
        new_stmts
    }

    fn transform_stmt(&mut self, stmt: Stmt) -> Vec<Stmt> {
        match stmt {
            Stmt::Let(name, expr, comment) => {
                let offset = self.scope_offset;
                self.var_map.insert(name.clone(), offset);
                self.scope_offset += 1;

                let set_call = Stmt::Expr(
                    Expr::Call(
                        "stack_set".to_string(),
                        vec![
                            Expr::Variable(self.base_var.clone()),
                            Expr::Number(offset as f64),
                            self.transform_expr(expr),
                        ],
                    ),
                    comment,
                );
                vec![set_call]
            }
            Stmt::Assign(name, expr, comment) => {
                if let Some(&offset) = self.var_map.get(&name) {
                    vec![Stmt::Expr(
                        Expr::Call(
                            "stack_set".to_string(),
                            vec![
                                Expr::Variable(self.base_var.clone()),
                                Expr::Number(offset as f64),
                                self.transform_expr(expr),
                            ],
                        ),
                        comment,
                    )]
                } else {
                    vec![Stmt::Assign(name, self.transform_expr(expr), comment)]
                }
            }
            Stmt::Expr(expr, comment) => vec![Stmt::Expr(self.transform_expr(expr), comment)],
            Stmt::If(cond, t, e, comment) => {
                vec![Stmt::If(
                    self.transform_expr(cond),
                    self.transform_scoped(t),
                    e.map(|b| self.transform_scoped(b)),
                    comment,
                )]
            }
            Stmt::Repeat(c, b, comment) => vec![Stmt::Repeat(
                self.transform_expr(c),
                self.transform_scoped(b),
                comment,
            )],
            Stmt::Forever(b, comment) => vec![Stmt::Forever(self.transform_scoped(b), comment)],
            Stmt::Until(c, b, comment) => vec![Stmt::Until(
                self.transform_expr(c),
                self.transform_scoped(b),
                comment,
            )],
            Stmt::Return(e, comment) => {
                let free_call = Stmt::Expr(
                    Expr::Call(
                        "sys_free".to_string(),
                        vec![Expr::Variable(self.base_var.clone())],
                    ),
                    None,
                );
                vec![
                    free_call,
                    Stmt::Return(e.map(|x| self.transform_expr(x)), comment),
                ]
            }
            Stmt::Match(cond, cases, def, comment) => {
                let new_cases = cases
                    .into_iter()
                    .map(|(e, b)| (self.transform_expr(e), self.transform_scoped(b)))
                    .collect();
                let new_def = def.map(|b| self.transform_scoped(b));
                vec![Stmt::Match(
                    self.transform_expr(cond),
                    new_cases,
                    new_def,
                    comment,
                )]
            }
            Stmt::CBlock(name, args, body, comment) => {
                let new_args = args.into_iter().map(|a| self.transform_expr(a)).collect();
                vec![Stmt::CBlock(
                    name,
                    new_args,
                    self.transform_scoped(body),
                    comment,
                )]
            }
            _ => vec![stmt],
        }
    }

    fn transform_expr(&self, expr: Expr) -> Expr {
        match expr {
            Expr::Variable(name) => {
                if let Some(&offset) = self.var_map.get(&name) {
                    Expr::Call(
                        "item_of_list".to_string(),
                        vec![
                            Expr::Variable("_RAM".to_string()),
                            Expr::BinOp(
                                Box::new(Expr::Variable(self.base_var.clone())),
                                Op::Add,
                                Box::new(Expr::Number(offset as f64)),
                            ),
                        ],
                    )
                } else {
                    Expr::Variable(name)
                }
            }
            Expr::BinOp(l, op, r) => Expr::BinOp(
                Box::new(self.transform_expr(*l)),
                op,
                Box::new(self.transform_expr(*r)),
            ),
            Expr::UnOp(op, v) => Expr::UnOp(op, Box::new(self.transform_expr(*v))),
            Expr::Call(n, args) => Expr::Call(
                n,
                args.into_iter().map(|a| self.transform_expr(a)).collect(),
            ),
            Expr::List(l) => Expr::List(l.into_iter().map(|a| self.transform_expr(a)).collect()),
            _ => expr,
        }
    }
}
