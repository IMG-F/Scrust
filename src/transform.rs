use crate::ast::*;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};

static PROC_COUNTER: AtomicU64 = AtomicU64::new(0);
fn get_unique_id() -> String {
    let val = PROC_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    format!("pkg_proc_{}", val)
}

fn qualify_calls(stmts: &mut [Stmt], pkg_name: &str, pkg_procs: &HashSet<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Expr(e, _)
            | Stmt::Let(_, e, _)
            | Stmt::Assign(_, e, _)
            | Stmt::Return(Some(e), _) => {
                qualify_calls_in_expr(e, pkg_name, pkg_procs);
            }
            Stmt::If(c, t, e, _) => {
                qualify_calls_in_expr(c, pkg_name, pkg_procs);
                qualify_calls(t, pkg_name, pkg_procs);
                if let Some(eb) = e {
                    qualify_calls(eb, pkg_name, pkg_procs);
                }
            }
            Stmt::Repeat(c, b, _) => {
                qualify_calls_in_expr(c, pkg_name, pkg_procs);
                qualify_calls(b, pkg_name, pkg_procs);
            }
            Stmt::Forever(b, _) => qualify_calls(b, pkg_name, pkg_procs),
            Stmt::Until(c, b, _) => {
                qualify_calls_in_expr(c, pkg_name, pkg_procs);
                qualify_calls(b, pkg_name, pkg_procs);
            }
            Stmt::Match(expr, arms, else_block, _) => {
                qualify_calls_in_expr(expr, pkg_name, pkg_procs);
                for (pat, stmts) in arms {
                    qualify_calls_in_expr(pat, pkg_name, pkg_procs);
                    qualify_calls(stmts, pkg_name, pkg_procs);
                }
                if let Some(eb) = else_block {
                    qualify_calls(eb, pkg_name, pkg_procs);
                }
            }
            Stmt::CBlock(_, args, stmts, _) => {
                for arg in args {
                    qualify_calls_in_expr(arg, pkg_name, pkg_procs);
                }
                qualify_calls(stmts, pkg_name, pkg_procs);
            }
            Stmt::Return(None, _) | Stmt::Comment(_) => {}
        }
    }
}

fn qualify_calls_in_expr(expr: &mut Expr, pkg_name: &str, pkg_procs: &HashSet<String>) {
    match expr {
        Expr::Call(name, args) => {
            if pkg_procs.contains(name) {
                *name = format!("{}::{}", pkg_name, name);
            }
            for arg in args {
                qualify_calls_in_expr(arg, pkg_name, pkg_procs);
            }
        }
        Expr::BinOp(l, _, r) => {
            qualify_calls_in_expr(l, pkg_name, pkg_procs);
            qualify_calls_in_expr(r, pkg_name, pkg_procs);
        }
        Expr::UnOp(_, e) => qualify_calls_in_expr(e, pkg_name, pkg_procs),
        Expr::List(items) => {
            for item in items {
                qualify_calls_in_expr(item, pkg_name, pkg_procs);
            }
        }
        _ => {}
    }
}

pub fn transform_program(program: &mut Program, packages: &HashMap<String, Package>) {
    let has_ram = program
        .items
        .iter()
        .any(|i| matches!(i, Item::Variable(v) if v.name == "_RAM"));

    // 0. Merge used package procedures (Zero-overhead tree shaking)
    let mut pending_scan = Vec::new();
    // Map from "pkg::proc" to "random_unique_id"
    let mut package_proc_mapping: HashMap<String, String> = HashMap::new();

    // Initial scan of main program
    for item in &program.items {
        scan_item_for_calls(item, &mut pending_scan);
    }

    let mut merged_items = Vec::new();
    let mut processed_package_procs = HashSet::new();

    while let Some(call_name) = pending_scan.pop() {
        if call_name.contains("::") {
            // It's likely a package call
            let parts: Vec<&str> = call_name.split("::").collect();
            if parts.len() == 2 {
                let pkg_name = parts[0];
                let proc_name = parts[1];

                let full_name = call_name.clone();
                if processed_package_procs.contains(&full_name) {
                    continue;
                }

                if let Some(pkg) = packages.get(pkg_name) {
                    // Find the proc in the package
                    if let Some(item) = pkg.items.iter().find(|i| {
                        if let Item::Procedure(p) = i {
                            p.name == proc_name
                        } else {
                            false
                        }
                    }) {
                        if let Item::Procedure(proc) = item {
                            processed_package_procs.insert(full_name.clone());

                            // Generate unique name
                            let unique_name = get_unique_id();
                            package_proc_mapping.insert(full_name.clone(), unique_name.clone());

                            let mut new_proc = proc.clone();
                            new_proc.name = unique_name.clone();
                            
                            // Collect package proc names
                            let pkg_procs: HashSet<String> = pkg.items.iter().filter_map(|i| {
                                if let Item::Procedure(p) = i {
                                    Some(p.name.clone())
                                } else {
                                    None
                                }
                            }).collect();
                            
                            // Qualify calls
                            qualify_calls(&mut new_proc.body, pkg_name, &pkg_procs);

                            // Scan this new procedure for more calls
                            scan_stmts_for_calls(&new_proc.body, &mut pending_scan);

                            merged_items.push(Item::Procedure(new_proc));
                        }
                    }
                }
            }
        }
    }

    // Apply renaming to the whole program (including merged items)
    // We need to rename calls in:
    // 1. Original program items
    // 2. Merged items (if they call other package procs)

    // Helper to rename calls in statements
    fn rename_calls_in_stmts(stmts: &mut [Stmt], mapping: &HashMap<String, String>) {
        for stmt in stmts {
            match stmt {
                Stmt::Expr(e, _)
                | Stmt::Let(_, e, _)
                | Stmt::Assign(_, e, _)
                | Stmt::Return(Some(e), _) => {
                    rename_calls_in_expr(e, mapping);
                }
                Stmt::If(c, t, e, _) => {
                    rename_calls_in_expr(c, mapping);
                    rename_calls_in_stmts(t, mapping);
                    if let Some(eb) = e {
                        rename_calls_in_stmts(eb, mapping);
                    }
                }
                Stmt::Repeat(c, b, _) => {
                    rename_calls_in_expr(c, mapping);
                    rename_calls_in_stmts(b, mapping);
                }
                Stmt::Forever(b, _) => rename_calls_in_stmts(b, mapping),
                Stmt::Until(c, b, _) => {
                    rename_calls_in_expr(c, mapping);
                    rename_calls_in_stmts(b, mapping);
                }
                _ => {}
            }
        }
    }

    fn rename_calls_in_expr(expr: &mut Expr, mapping: &HashMap<String, String>) {
        match expr {
            Expr::Call(name, args) => {
                if let Some(new_name) = mapping.get(name) {
                    *name = new_name.clone();
                }
                for arg in args {
                    rename_calls_in_expr(arg, mapping);
                }
            }
            Expr::BinOp(l, _, r) => {
                rename_calls_in_expr(l, mapping);
                rename_calls_in_expr(r, mapping);
            }
            _ => {}
        }
    }

    // Apply renaming
    for item in &mut program.items {
        match item {
            Item::Procedure(p) => rename_calls_in_stmts(&mut p.body, &package_proc_mapping),
            Item::Function(f) => rename_calls_in_stmts(&mut f.body, &package_proc_mapping),
            _ => {}
        }
    }
    for item in &mut merged_items {
        if let Item::Procedure(p) = item {
            rename_calls_in_stmts(&mut p.body, &package_proc_mapping);
        }
    }

    program.items.extend(merged_items);

    // Remove `use` statements
    program.items.retain(|item| !matches!(item, Item::Use(_)));

    let mut new_items = Vec::new();

    // Collect user procedures for call flattening (now includes merged package procs)
    let mut user_procs = HashSet::new();
    let mut value_procs = HashSet::new();
    for item in &program.items {
        if let Item::Procedure(proc) = item {
            if !is_helper(&proc.name) {
                user_procs.insert(proc.name.clone());
                if proc.return_type.is_some() {
                    value_procs.insert(proc.name.clone());
                }
            }
        }
    }

    // Check if we need to transform anything
    let needs_transform = program.items.iter().any(|item| match item {
        Item::Procedure(proc) => should_transform_proc(proc, &value_procs),
        Item::Function(func) => should_transform_func(func, &value_procs),
        _ => false,
    });

    // 1. Inject Global Resources (Only if needed)
    if !has_ram && needs_transform {
        new_items.push(Item::Variable(VariableDecl {
            name: "_RAM".to_string(),
            ty: Type::List,
            init: Expr::List(vec![]),
            visibility: Visibility::Public,
            comment: None,
        }));
        new_items.push(Item::Variable(VariableDecl {
            name: "_FREE_PAGES".to_string(),
            ty: Type::List,
            init: Expr::List(vec![]),
            visibility: Visibility::Public,
            comment: None,
        }));
        new_items.push(Item::Variable(VariableDecl {
            name: "_HIGH_WATER".to_string(),
            ty: Type::Number,
            init: Expr::Number(1.0),
            visibility: Visibility::Public,
            comment: None,
        }));
        new_items.push(Item::Variable(VariableDecl {
            name: "_RET_VAL".to_string(),
            ty: Type::Number,
            init: Expr::Number(0.0),
            visibility: Visibility::Public,
            comment: None,
        }));

        // 2. Inject Helper Procedures
        // sys_alloc
        new_items.push(Item::Procedure(ProcedureDef {
            name: "sys_alloc".to_string(),
            params: vec![],
            body: vec![Stmt::If(
                Expr::BinOp(
                    Box::new(Expr::Call(
                        "length_of_list".to_string(),
                        vec![Expr::String("_FREE_PAGES".to_string())],
                    )),
                    Op::Gt,
                    Box::new(Expr::Number(0.0)),
                ),
                vec![
                    Stmt::Assign(
                        "_RET_VAL".to_string(),
                        Expr::Call(
                            "item_of_list".to_string(),
                            vec![
                                Expr::String("_FREE_PAGES".to_string()),
                                Expr::Call(
                                    "length_of_list".to_string(),
                                    vec![Expr::String("_FREE_PAGES".to_string())],
                                ),
                            ],
                        ),
                        None,
                    ),
                    Stmt::Expr(
                        Expr::Call(
                            "delete_of_list".to_string(),
                            vec![
                                Expr::String("_FREE_PAGES".to_string()),
                                Expr::Call(
                                    "length_of_list".to_string(),
                                    vec![Expr::String("_FREE_PAGES".to_string())],
                                ),
                            ],
                        ),
                        None,
                    ),
                ],
                Some(vec![
                    Stmt::Assign(
                        "_RET_VAL".to_string(),
                        Expr::Variable("_HIGH_WATER".to_string()),
                        None,
                    ),
                    Stmt::Assign(
                        "_HIGH_WATER".to_string(),
                        Expr::BinOp(
                            Box::new(Expr::Variable("_HIGH_WATER".to_string())),
                            Op::Add,
                            Box::new(Expr::Number(16.0)),
                        ),
                        None,
                    ),
                    Stmt::Repeat(
                        Expr::Number(16.0),
                        vec![Stmt::Expr(
                            Expr::Call(
                                "add_to_list".to_string(),
                                vec![Expr::String("_RAM".to_string()), Expr::Number(0.0)],
                            ),
                            None,
                        )],
                        None,
                    ),
                ]),
                None,
            )],
            return_type: None,
            is_warp: true,
            comment: None, // No generated comments
        }));

        // sys_free
        new_items.push(Item::Procedure(ProcedureDef {
            name: "sys_free".to_string(),
            params: vec![Param {
                name: "ptr".to_string(),
                ty: Type::Number,
            }],
            body: vec![Stmt::Expr(
                Expr::Call(
                    "add_to_list".to_string(),
                    vec![
                        Expr::String("_FREE_PAGES".to_string()),
                        Expr::Variable("ptr".to_string()),
                    ],
                ),
                None,
            )],
            return_type: None,
            is_warp: true,
            comment: None,
        }));

        // stack_set
        new_items.push(Item::Procedure(ProcedureDef {
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
                }, // Scratch variables are dynamic, but type required
            ],
            body: vec![Stmt::Expr(
                Expr::Call(
                    "replace_item_of_list".to_string(),
                    vec![
                        Expr::String("_RAM".to_string()),
                        Expr::BinOp(
                            Box::new(Expr::Variable("ptr".to_string())),
                            Op::Add,
                            Box::new(Expr::Variable("offset".to_string())),
                        ),
                        Expr::Variable("val".to_string()),
                    ],
                ),
                None,
            )],
            return_type: None,
            is_warp: true,
            comment: None,
        }));
    }

    // 3. Transform User Items
    for item in program.items.drain(..) {
        match item {
            Item::Procedure(proc) => {
                if is_helper(&proc.name) {
                    new_items.push(Item::Procedure(proc));
                    continue;
                }
                if should_transform_proc(&proc, &value_procs) {
                    transform_procedure(proc, &value_procs, &mut new_items);
                } else {
                    new_items.push(Item::Procedure(proc));
                }
            }
            Item::Function(func) => {
                if should_transform_func(&func, &value_procs) {
                    transform_function(func, &value_procs, &mut new_items);
                } else {
                    new_items.push(Item::Function(func));
                }
            }
            _ => new_items.push(item),
        }
    }

    program.items = new_items;
}

fn is_helper(name: &str) -> bool {
    matches!(name, "sys_alloc" | "sys_free" | "stack_set")
}

fn scan_item_for_calls(item: &Item, calls: &mut Vec<String>) {
    match item {
        Item::Procedure(p) => scan_stmts_for_calls(&p.body, calls),
        Item::Function(f) => scan_stmts_for_calls(&f.body, calls),
        _ => {}
    }
}

fn scan_stmts_for_calls(stmts: &[Stmt], calls: &mut Vec<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Expr(e, _)
            | Stmt::Let(_, e, _)
            | Stmt::Assign(_, e, _)
            | Stmt::Return(Some(e), _) => {
                scan_expr_for_calls(e, calls);
            }
            Stmt::If(c, t, e, _) => {
                scan_expr_for_calls(c, calls);
                scan_stmts_for_calls(t, calls);
                if let Some(eb) = e {
                    scan_stmts_for_calls(eb, calls);
                }
            }
            Stmt::Repeat(c, b, _) => {
                scan_expr_for_calls(c, calls);
                scan_stmts_for_calls(b, calls);
            }
            Stmt::Forever(b, _) => scan_stmts_for_calls(b, calls),
            Stmt::Until(c, b, _) => {
                scan_expr_for_calls(c, calls);
                scan_stmts_for_calls(b, calls);
            }
            _ => {}
        }
    }
}

fn scan_expr_for_calls(expr: &Expr, calls: &mut Vec<String>) {
    match expr {
        Expr::Call(name, args) => {
            calls.push(name.clone());
            for arg in args {
                scan_expr_for_calls(arg, calls);
            }
        }
        Expr::BinOp(l, _, r) => {
            scan_expr_for_calls(l, calls);
            scan_expr_for_calls(r, calls);
        }
        _ => {}
    }
}

fn should_transform_proc(proc: &ProcedureDef, value_procs: &HashSet<String>) -> bool {
    proc.return_type.is_some()
        || contains_let_or_return(&proc.body)
        || contains_call_to_value_proc(&proc.body, value_procs)
}

fn should_transform_func(func: &Function, value_procs: &HashSet<String>) -> bool {
    contains_let_or_return(&func.body) || contains_call_to_value_proc(&func.body, value_procs)
}

fn contains_call_to_value_proc(stmts: &[Stmt], value_procs: &HashSet<String>) -> bool {
    let mut calls = Vec::new();
    scan_stmts_for_calls(stmts, &mut calls);
    calls.iter().any(|name| value_procs.contains(name))
}

fn contains_let_or_return(stmts: &[Stmt]) -> bool {
    for stmt in stmts {
        match stmt {
            Stmt::Let(_, _, _) | Stmt::Return(_, _) => return true,
            Stmt::If(_, t, e, _) => {
                if contains_let_or_return(t) {
                    return true;
                }
                if let Some(b) = e {
                    if contains_let_or_return(b) {
                        return true;
                    }
                }
            }
            Stmt::Repeat(_, b, _) => {
                if contains_let_or_return(b) {
                    return true;
                }
            }
            Stmt::Forever(b, _) => {
                if contains_let_or_return(b) {
                    return true;
                }
            }
            Stmt::Until(_, b, _) => {
                if contains_let_or_return(b) {
                    return true;
                }
            }
            Stmt::Match(_, cases, default, _) => {
                for (_, b) in cases {
                    if contains_let_or_return(b) {
                        return true;
                    }
                }
                if let Some(b) = default {
                    if contains_let_or_return(b) {
                        return true;
                    }
                }
            }
            Stmt::CBlock(_, _, b, _) => {
                if contains_let_or_return(b) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn transform_procedure(proc: ProcedureDef, user_procs: &HashSet<String>, out: &mut Vec<Item>) {
    let inner_name = format!("_inner_{}", proc.name);

    // Wrapper
    let mut wrapper_body = Vec::new();
    wrapper_body.push(Stmt::Expr(
        Expr::Call("sys_alloc".to_string(), vec![]),
        None,
    ));

    let mut inner_args = vec![Expr::Variable("_RET_VAL".to_string())];
    for param in &proc.params {
        inner_args.push(Expr::Variable(param.name.clone()));
    }

    wrapper_body.push(Stmt::Expr(Expr::Call(inner_name.clone(), inner_args), None));

    out.push(Item::Procedure(ProcedureDef {
        name: proc.name.clone(),
        params: proc.params.clone(),
        body: wrapper_body,
        return_type: None,     // Wrapper doesn't return
        is_warp: proc.is_warp, // Wrapper inherits warp status? Usually alloc/free are warp, but wrapper might not be if inner isn't.
        comment: proc.comment.clone(),
    }));

    // Inner
    let mut inner_params = vec![Param {
        name: "base".to_string(),
        ty: Type::Number,
    }];
    inner_params.extend(proc.params.clone());

    let mut ctx = TransformContext::new(user_procs.clone());
    let transformed_body = transform_stmts(proc.body, &mut ctx);

    let mut final_body = transformed_body;
    final_body.push(Stmt::Assign(
        "_RET_VAL".to_string(),
        Expr::Variable("base".to_string()),
        None,
    ));

    out.push(Item::Procedure(ProcedureDef {
        name: inner_name,
        params: inner_params,
        body: final_body,
        return_type: None,
        is_warp: proc.is_warp,
        comment: None,
    }));
}

fn transform_function(func: Function, user_procs: &HashSet<String>, out: &mut Vec<Item>) {
    let inner_name = format!("_inner_{}", func.name);

    // Wrapper (Event Handler)
    let mut wrapper_body = Vec::new();
    wrapper_body.push(Stmt::Expr(
        Expr::Call("sys_alloc".to_string(), vec![]),
        None,
    ));

    let mut inner_args = vec![Expr::Variable("_RET_VAL".to_string())];
    for param in &func.params {
        inner_args.push(Expr::Variable(param.name.clone()));
    }

    wrapper_body.push(Stmt::Expr(Expr::Call(inner_name.clone(), inner_args), None));

    out.push(Item::Function(Function {
        name: func.name.clone(),
        attributes: func.attributes.clone(),
        params: func.params.clone(),
        body: wrapper_body,
        is_warp: func.is_warp,
        comment: func.comment.clone(),
    }));

    // Inner
    let mut inner_params = vec![Param {
        name: "base".to_string(),
        ty: Type::Number,
    }];
    inner_params.extend(func.params.clone());

    let mut ctx = TransformContext::new(user_procs.clone());
    let mut final_body = transform_stmts(func.body, &mut ctx);

    if let Item::Function(f) = out.last_mut().unwrap() {
        f.body.push(Stmt::Expr(
            Expr::Call(
                "sys_free".to_string(),
                vec![Expr::Variable("_RET_VAL".to_string())],
            ),
            None,
        ));
    }

    final_body.push(Stmt::Assign(
        "_RET_VAL".to_string(),
        Expr::Variable("base".to_string()),
        None,
    ));

    out.push(Item::Procedure(ProcedureDef {
        name: inner_name,
        params: inner_params,
        body: final_body,
        return_type: None,
        is_warp: func.is_warp,
        comment: None,
    }));
}

struct TransformContext {
    scope_offset: i32,
    locals: HashMap<String, i32>, // name -> offset
    scopes: Vec<Vec<String>>,     // stack of scopes (list of vars in each scope)
    user_procs: HashSet<String>,
}

impl TransformContext {
    fn new(user_procs: HashSet<String>) -> Self {
        Self {
            scope_offset: 1, // Start at 1? 0 is return slot. So 1.
            locals: HashMap::new(),
            scopes: vec![Vec::new()],
            user_procs,
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn leave_scope(&mut self) {
        let vars = self.scopes.pop().unwrap();
        for var in vars {
            self.locals.remove(&var);
        }
    }

    fn define_local(&mut self, name: String) -> i32 {
        let offset = self.scope_offset;
        self.scope_offset += 1;
        self.locals.insert(name.clone(), offset);
        self.scopes.last_mut().unwrap().push(name);
        offset
    }

    fn define_temp(&mut self) -> i32 {
        let offset = self.scope_offset;
        self.scope_offset += 1;
        offset
    }

    fn get_local(&self, name: &str) -> Option<i32> {
        self.locals.get(name).cloned()
    }
}

fn transform_stmts(stmts: Vec<Stmt>, ctx: &mut TransformContext) -> Vec<Stmt> {
    let mut new_stmts = Vec::new();

    for stmt in stmts {
        let mut pre_stmts = Vec::new();
        match stmt {
            Stmt::Let(name, expr, _) => {
                let offset = ctx.define_local(name);
                let transformed_expr = process_expr(expr, &mut pre_stmts, ctx);
                new_stmts.extend(pre_stmts);
                new_stmts.push(Stmt::Expr(
                    Expr::Call(
                        "stack_set".to_string(),
                        vec![
                            Expr::Variable("base".to_string()),
                            Expr::Number(offset as f64),
                            transformed_expr,
                        ],
                    ),
                    None,
                ));
            }
            Stmt::Return(expr, _) => {
                let ret_val = if let Some(e) = expr {
                    process_expr(e, &mut pre_stmts, ctx)
                } else {
                    Expr::Number(0.0)
                };
                new_stmts.extend(pre_stmts);
                new_stmts.push(Stmt::Expr(
                    Expr::Call(
                        "stack_set".to_string(),
                        vec![
                            Expr::Variable("base".to_string()),
                            Expr::Number(0.0), // Return slot is 0
                            ret_val,
                        ],
                    ),
                    None,
                ));
                new_stmts.push(Stmt::Assign(
                    "_RET_VAL".to_string(),
                    Expr::Variable("base".to_string()),
                    None,
                ));
                // We don't stop script.
            }
            Stmt::Assign(name, expr, _) => {
                let transformed_expr = process_expr(expr, &mut pre_stmts, ctx);
                new_stmts.extend(pre_stmts);
                if let Some(offset) = ctx.get_local(&name) {
                    new_stmts.push(Stmt::Expr(
                        Expr::Call(
                            "stack_set".to_string(),
                            vec![
                                Expr::Variable("base".to_string()),
                                Expr::Number(offset as f64),
                                transformed_expr,
                            ],
                        ),
                        None,
                    ));
                } else {
                    new_stmts.push(Stmt::Assign(name, transformed_expr, None));
                }
            }
            Stmt::Expr(expr, _) => {
                let transformed_expr = process_expr(expr, &mut pre_stmts, ctx);
                new_stmts.extend(pre_stmts);
                new_stmts.push(Stmt::Expr(transformed_expr, None));
            }
            Stmt::If(cond, then_block, else_block, _) => {
                let t_cond = process_expr(cond, &mut pre_stmts, ctx);
                new_stmts.extend(pre_stmts);

                ctx.enter_scope();
                let saved = ctx.scope_offset;
                let t_then = transform_stmts(then_block, ctx);
                ctx.leave_scope();
                ctx.scope_offset = saved;

                let t_else = if let Some(else_b) = else_block {
                    ctx.enter_scope();
                    let saved = ctx.scope_offset;
                    let b = transform_stmts(else_b, ctx);
                    ctx.leave_scope();
                    ctx.scope_offset = saved;
                    Some(b)
                } else {
                    None
                };

                new_stmts.push(Stmt::If(t_cond, t_then, t_else, None));
            }
            Stmt::Repeat(count, body, _) => {
                let t_count = process_expr(count, &mut pre_stmts, ctx);
                new_stmts.extend(pre_stmts);

                ctx.enter_scope();
                let saved = ctx.scope_offset;
                let t_body = transform_stmts(body, ctx);
                ctx.leave_scope();
                ctx.scope_offset = saved;
                new_stmts.push(Stmt::Repeat(t_count, t_body, None));
            }
            Stmt::Forever(body, _) => {
                ctx.enter_scope();
                let saved = ctx.scope_offset;
                let t_body = transform_stmts(body, ctx);
                ctx.leave_scope();
                ctx.scope_offset = saved;
                new_stmts.push(Stmt::Forever(t_body, None));
            }
            Stmt::Until(cond, body, _) => {
                let t_cond = process_expr(cond, &mut pre_stmts, ctx);
                new_stmts.extend(pre_stmts);

                ctx.enter_scope();
                let saved = ctx.scope_offset;
                let t_body = transform_stmts(body, ctx);
                ctx.leave_scope();
                ctx.scope_offset = saved;
                new_stmts.push(Stmt::Until(t_cond, t_body, None));
            }
            _ => new_stmts.push(stmt),
        }
    }

    new_stmts
}

fn process_expr(expr: Expr, pre_stmts: &mut Vec<Stmt>, ctx: &mut TransformContext) -> Expr {
    match expr {
        Expr::Variable(name) => {
            if let Some(offset) = ctx.get_local(&name) {
                Expr::Call(
                    "item_of_list".to_string(),
                    vec![
                        Expr::String("_RAM".to_string()),
                        Expr::BinOp(
                            Box::new(Expr::Variable("base".to_string())),
                            Op::Add,
                            Box::new(Expr::Number(offset as f64)),
                        ),
                    ],
                )
            } else {
                Expr::Variable(name)
            }
        }
        Expr::Call(name, args) => {
            let new_args: Vec<Expr> = args
                .into_iter()
                .map(|a| process_expr(a, pre_stmts, ctx))
                .collect();

            if ctx.user_procs.contains(&name) {
                // Flatten logic
                // 1. Emit call
                pre_stmts.push(Stmt::Expr(Expr::Call(name.clone(), new_args), None));

                // 2. Alloc temp
                let temp_offset = ctx.define_temp();

                // 3. Save result
                pre_stmts.push(Stmt::Expr(
                    Expr::Call(
                        "stack_set".to_string(),
                        vec![
                            Expr::Variable("base".to_string()),
                            Expr::Number(temp_offset as f64),
                            Expr::Call(
                                "item_of_list".to_string(),
                                vec![
                                    Expr::String("_RAM".to_string()),
                                    Expr::BinOp(
                                        Box::new(Expr::Variable("_RET_VAL".to_string())),
                                        Op::Add,
                                        Box::new(Expr::Number(0.0)),
                                    ),
                                ],
                            ),
                        ],
                    ),
                    None,
                ));

                // 4. Free result page
                pre_stmts.push(Stmt::Expr(
                    Expr::Call(
                        "sys_free".to_string(),
                        vec![Expr::Variable("_RET_VAL".to_string())],
                    ),
                    None,
                ));

                // 5. Return access
                Expr::Call(
                    "item_of_list".to_string(),
                    vec![
                        Expr::String("_RAM".to_string()),
                        Expr::BinOp(
                            Box::new(Expr::Variable("base".to_string())),
                            Op::Add,
                            Box::new(Expr::Number(temp_offset as f64)),
                        ),
                    ],
                )
            } else {
                Expr::Call(name, new_args)
            }
        }
        Expr::BinOp(l, op, r) => {
            let l_new = process_expr(*l, pre_stmts, ctx);
            let r_new = process_expr(*r, pre_stmts, ctx);
            Expr::BinOp(Box::new(l_new), op, Box::new(r_new))
        }
        _ => expr,
    }
}
