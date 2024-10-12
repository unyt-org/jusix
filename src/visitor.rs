use swc_core::{
    atoms::Atom,
    common::{util::take::Take, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            ArrowExpr, AwaitExpr, BindingIdent, BlockStmt, BlockStmtOrExpr, CallExpr, Callee, CatchClause, ClassDecl, ClassMethod, Constructor, Expr, ExprOrSpread, ExprStmt, FnDecl, FnExpr, Id, Ident, JSXAttr, JSXAttrName, JSXAttrValue, JSXElement, JSXElementChild, JSXElementName, JSXEmptyExpr, JSXExpr, JSXExprContainer, JSXSpreadChild, JSXText, Lit, MemberProp, Null, Number, ObjectPatProp, Param, ParamOrTsParamProp, Pat, PrivateMethod, ReturnStmt, Stmt, Str, ThisExpr, TsAsExpr, TsEnumDecl, TsInterfaceDecl, TsParamPropParam, TsType, TsTypeAliasDecl, VarDecl
        },
        visit::{Fold, FoldWith, Visit, VisitWith},
    },
};



fn collect_params(params: &Vec<Pat>, add_this: bool) -> Vec<Id> {
    let mut result: Vec<Id> = vec![];

    if add_this {
        let this = (Atom::from("this"), SyntaxContext::empty());
        result.push(this);
    }

    for param in params {
        match param {
            Pat::Ident(i) => {
                let var = i.to_id();
                if !result.contains(&var) {
                    result.push(var);
                }
            }
            Pat::Array(a) => {
                for elem in &a.elems {
                    match elem {
                        Some(Pat::Ident(i)) => {
                            let var = i.to_id();
                            if !result.contains(&var) {
                                result.push(var);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Pat::Object(o) => {
                for prop in &o.props {
                    match prop {
                        ObjectPatProp::KeyValue(kv) => {
                            match *kv.value.clone() {
                                Pat::Ident(i) => {
                                    let var = i.to_id();
                                    if !result.contains(&var) {
                                        result.push(var);
                                    }
                                }
                                _ => {}
                            }
                        }
                        ObjectPatProp::Assign(a) => {
                            let var = a.key.to_id();
                            if !result.contains(&var) {
                                result.push(var);
                            }
                        }
                        ObjectPatProp::Rest(r) => {
                            if let Pat::Ident(i) = *r.arg.clone() {
                                let var: (Atom, SyntaxContext) = i.to_id();
                                if !result.contains(&var) {
                                    result.push(var);
                                }
                            }
                        }
                    }
                }
            }
            Pat::Rest(r) => {
                if let Pat::Ident(i) = *r.arg.clone() {
                    let var = i.to_id();
                    if !result.contains(&var) {
                        result.push(var);
                    }
                }
            }
            _ => {}
        }
    }

    return result;
}

struct VariableCollector {
    has_custom_use: bool,
    used_variables: Vec<Id>,
    variable_declarations: Vec<Id>,
}

impl VariableCollector {
    fn new() -> Self {
        VariableCollector {
            has_custom_use: false,
            used_variables: Vec::new(),
            variable_declarations: Vec::new(),
        }
    }
}

impl VariableCollector {

    /**
     * Visit a block or function body recursively
     */
    fn visit_block_recursive(&mut self, params: &Vec<Pat>, visitable: &impl VisitWith<dyn Visit>, has_this: bool) {
        // recursively visit the arrow function body with a new collector
        let mut collector = VariableCollector::new();

        // add existing variable declarations to the new collector
        for var in &self.variable_declarations {
            if !collector.variable_declarations.contains(var) {
                collector.variable_declarations.push(var.clone());
            }
        }

        // add function parameters to the new collector
        for param in collect_params(params, has_this) {
            if !collector.variable_declarations.contains(&param) {
                collector.variable_declarations.push(param);
            }
        }

        visitable.visit_children_with(&mut collector);

        // TODO: this should not be necessary
        if collector.has_custom_use {
            self.has_custom_use = true;
        }

        // add used variables to the current collector
        for var in collector.used_variables {
            if !self.used_variables.contains(&var) {
                self.used_variables.push(var);
            }
        }
    }
}

impl Visit for VariableCollector {

    // when ecountering existing custom use() call, don't inject use() call
    fn visit_call_expr(&mut self, call_expr: &CallExpr) {
        match &call_expr.callee {
            Callee::Expr(e) => {
                match e.as_ident() {
                    Some(i) => {
                        if i.sym.eq_ignore_ascii_case("use") {
                            self.has_custom_use = true;
                            return;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
            
        }
        call_expr.visit_children_with(self);
    }

    fn visit_ts_type(&mut self, node: &TsType) {
        // ignore type annotations
    }

    fn visit_ts_type_alias_decl(&mut self, node: &TsTypeAliasDecl) {
        // ignore type alias declarations
    }

    fn visit_ts_interface_decl(&mut self, node: &TsInterfaceDecl) {
        // ignore interface declarations
    }

    fn visit_ts_enum_decl(&mut self, node: &TsEnumDecl) {
        // remember enum declaration name
        if !self.variable_declarations.contains(&node.id.to_id()) {
            self.variable_declarations.push(node.id.to_id());
        }
        // ignore enum declarations
    }

    fn visit_jsx_element_name(&mut self, _name: &JSXElementName) {
        // ignore jsx name as identifier
    }

    fn visit_number(&mut self, _node: &Number) {
        // ignore number literals
    }

    fn visit_this_expr(&mut self, _node: &ThisExpr) {
        // add 'this' to used variables
        let var = (Atom::from("this"), SyntaxContext::empty());
        if !self.used_variables.contains(&var) &&
            !self.variable_declarations.contains(&var) {
            self.used_variables.push(var);
        }
    }

    fn visit_ident(&mut self, ident: &Ident) {
        // add variable to list if not already present and not
        // in variable_declarations
        let name: String = ident.sym.to_string();
        let var = ident.to_id();
        if !self.used_variables.contains(&var) &&
            !self.variable_declarations.contains(&var) &&
            !GLOBAL_THIS_ALIASES.contains(&name.as_str()) && 
            !RESERVED_IDENTIFIERS.contains(&name.as_str()) {
            self.used_variables.push(var);
        }
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl) {
        for decl in &var_decl.decls {
            // add variable to list if not already present
            match &decl.name {
                Pat::Ident(i) => {
                    let var = i.to_id();
                    if !self.variable_declarations.contains(&var) {
                        self.variable_declarations.push(var);
                    }
                }
                Pat::Object(o) => {
                    for prop in &o.props {
                        match prop {
                            ObjectPatProp::KeyValue(kv) => {
                                match *kv.value.clone() {
                                    Pat::Ident(i) => {
                                        let var = i.to_id();
                                        if !self.variable_declarations.contains(&var) {
                                            self.variable_declarations.push(var);
                                        }
                                    }
                                    _ => {}
                                    
                                }
                            }
                            ObjectPatProp::Assign(a) => {
                                let var = a.key.to_id();
                                if !self.variable_declarations.contains(&var) {
                                    self.variable_declarations.push(var);
                                }
                            }
                            ObjectPatProp::Rest(r) => {
                                if let Pat::Ident(i) = &*r.arg {
                                    let var = i.to_id();
                                    if !self.variable_declarations.contains(&var) {
                                        self.variable_declarations.push(var);
                                    }
                                }
                            }
                        }
                    }
                }
                Pat::Array(a) => {
                    for elem in &a.elems {
                        match elem {
                            Some(Pat::Ident(i)) => {
                                let var = i.to_id();
                                if !self.variable_declarations.contains(&var) {
                                    self.variable_declarations.push(var);
                                }
                            }
                            Some(Pat::Array(a)) => {
                                for elem in &a.elems {
                                    match elem {
                                        Some(Pat::Ident(i)) => {
                                            let var = i.to_id();
                                            if !self.variable_declarations.contains(&var) {
                                                self.variable_declarations.push(var);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            decl.visit_with(self);
        }
    }

    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        // store name in variable_declarations
        let var = fn_decl.ident.to_id();
        if !self.variable_declarations.contains(&var) {
            self.variable_declarations.push(var);
        }

        // Visit the function body
        self.visit_block_recursive(
            &fn_decl.function.params.iter().map(|p| p.pat.clone()).collect(),
            &fn_decl.function.body,
            true
        );
    }

    fn visit_fn_expr(&mut self, node: &FnExpr) {
        // Visit the function body
        self.visit_block_recursive(
            &node.function.params.iter().map(|p| p.pat.clone()).collect(),
            &node.function.body,
            true
        );
    }

    fn visit_constructor(&mut self, node: &Constructor) {
        // Visit the constructor body
        self.visit_block_recursive(
            &node.params.iter().map(|p| match p {
                ParamOrTsParamProp::Param(p) => p.pat.clone(),
                ParamOrTsParamProp::TsParamProp(p) => match &p.param {
                    TsParamPropParam::Ident(i) => Pat::Ident(i.clone()),
                    TsParamPropParam::Assign(a) => Pat::Assign(a.clone()),
                }
            }).collect(),
            &node.body,
            true
        );
    }

    fn visit_class_method(&mut self, node: &ClassMethod) {       
        // Visit the class method body
        self.visit_block_recursive(
            &node.function.params.iter().map(|p| p.pat.clone()).collect(),
            &node.function.body,
            true
        );
    }

    fn visit_private_method(&mut self, node: &PrivateMethod) {
        // Visit the private method body
        self.visit_block_recursive(
            &node.function.params.iter().map(|p| p.pat.clone()).collect(),
            &node.function.body,
            true
        );
    }

    fn visit_arrow_expr(&mut self, node: &ArrowExpr) {
        // Visit function body recursively
        self.visit_block_recursive(&node.params, &node.body, false);
    }


    fn visit_class_decl(&mut self, node: &ClassDecl) {
        // store name in variable_declarations
        let var = node.ident.to_id();
        if !self.variable_declarations.contains(&var) {
            self.variable_declarations.push(var);
        }
        // Visit extended class
        if let Some(super_class) = &node.class.super_class {
            super_class.visit_with(self);
        }

        // Visit the class body
        self.visit_block_recursive(&vec![], &node.class.body, true);
    }

    fn visit_block_stmt(&mut self, block_stmt: &BlockStmt) {
        // Visit the block statement recursively
        self.visit_block_recursive(&vec![], block_stmt, false);
    }

    // catch expression, pass variables to block
    fn visit_catch_clause(&mut self, node: &CatchClause) {

        let mut params = vec![];

        if let Some(param) = &node.param {
            params.push(param.clone());
        }

        self.visit_block_recursive(&params, &node.body, false);
    }

}


struct AsyncChecker {
    is_async: bool,
}

impl Visit for AsyncChecker {
    fn visit_await_expr(&mut self, _node: &AwaitExpr) {
        self.is_async = true;
    }

    // ignore if async inside function
    fn visit_fn_decl(&mut self, _node: &FnDecl) {}
    fn visit_arrow_expr(&mut self, _node: &ArrowExpr) {}
    fn visit_fn_expr(&mut self, _node: &FnExpr) {}
}


const GLOBAL_THIS_ALIASES: [&'static str; 3] = [
    "globalThis",
    "self",
    "window",
];

const RESERVED_IDENTIFIERS: [&'static str; 4] = [
    "undefined",
    "NaN",
    "Infinity",
    "use"
];


const DOLLAR_METHODS: [&'static str; 3] = [
    "map",
    "filter",
    "reduce",
];

const RENDER_METHODS: [&'static str; 6] = [
    "renderBackend",
    "renderFrontend",
    "renderStatic",
    "renderDynamic",
    "renderHybrid",
    "renderPreview"
];


pub struct TransformVisitor;

impl TransformVisitor {
    // wraps in expression in always() if needed
    fn transform_expr_reactive(&mut self, e: Box<Expr>, always_fn_name: &str) -> Box<Expr> {
        match e.unwrap_parens() {
            // keep single literal values
            Expr::Lit(_) | Expr::JSXElement(_) | Expr::Ident(_) | Expr::This(_) => e,

            // keep functions
            Expr::Arrow(_) | Expr::Fn(_) => e,

            // has a $.x property, don't add always
            Expr::Member(m)
                if m.obj.is_member()
                    && (m.obj.as_member().unwrap().prop.is_ident_with("$")
                        || m.obj.as_member().unwrap().prop.is_ident_with("$$")) =>
            {
                e
            }

            // x.y property, convert to prop(x, 'y')
            Expr::Member(m) =>
                    Box::new(Expr::Call(
                        CallExpr {
                            span: DUMMY_SP,
                            callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                                "prop".into(),
                                DUMMY_SP,
                                Default::default(),
                            )))),
                            args: vec![
                                ExprOrSpread {
                                    expr: self.transform_expr_reactive(m.obj.clone(), "_$"),
                                    spread: None
                                },
                                // convert prop to string
                                match &m.prop {
                                    MemberProp::Ident(_) => Expr::Lit(Lit::Str(Str {
                                        span: DUMMY_SP,
                                        value: m.prop.as_ident().unwrap().sym.clone().into(),
                                        raw: None
                                    })).into(),
                                    MemberProp::Computed(e) => e.expr.clone().into(),
                                    MemberProp::PrivateName(_) => panic!("Private name not supported"),
                                }
                            ],
                            type_args: Take::dummy(),
                            ctxt: Default::default(),
                        }
                )
            ),
            
            // convert array.map(() => {}) to _$method(array, 'map', (() => {})
            // TODO
            Expr::Call(c)
                if c.callee.is_expr()
                    && c.callee.as_expr().unwrap().is_member()
                    // any DOLLAR_METHODS
                    && DOLLAR_METHODS.iter().any(|m| c.callee.as_expr().unwrap().as_member().unwrap().prop.is_ident_with(m)) =>
                    // && (c.callee.as_expr().unwrap().as_member().unwrap().prop.is_ident_with("map")) =>
            {
                let member = c.callee.as_expr().unwrap().as_member().unwrap();
                let obj = member.obj.clone();
                let prop = member.prop.clone();

                let mut args: Vec::<ExprOrSpread> = vec![
                    ExprOrSpread {
                        expr: obj,
                        spread: None
                    },
                    ExprOrSpread {
                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                            span: DUMMY_SP,
                            value: prop.as_ident().unwrap().sym.clone().into(),
                            raw: None
                        }))),
                        spread: None
                    }
                ];

                for arg in c.args.clone().into_iter().map(|a| {
                    match a {
                        ExprOrSpread {
                            expr: e,
                            spread: None,
                        } => match e.unwrap_parens() {
                            Expr::Arrow(a1) => ExprOrSpread {
                                expr: {
                                    let mut a2 = a1.clone();
                                    a2.body = Box::new(
                                       *a2.body.fold_with(self)
                                    );
                                    Box::new(Expr::Arrow(a2))
                                },
                                spread: None,
                            },
                            _ => ExprOrSpread {
                                expr: e,
                                spread: None,
                            }
                        },
                        _ => a
                    }
                }) {
                    args.push(arg);
                }


                Box::new(Expr::Call(CallExpr {
                    span: c.span,
                    // callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                    //     span: DUMMY_SP,
                    //     obj: obj,
                    //     prop: MemberProp::Ident(IdentName::from(
                    //         format!("$.{}", prop.as_ident().unwrap().sym).to_string()
                    //     )),
                    // }))),
                    callee: Callee::Expr(Box::new(Expr::Ident(
                        Ident::new(
                            "_$method".into(),
                            DUMMY_SP,
                            Default::default(),
                        )
                    ))),
                    // transform first arg if it's a function, keep others
                    args,
                    type_args: Take::dummy(),
                    ctxt: Default::default(),
                }))
            }


            // already has an always() or $$() wrapper
            Expr::Call(c)
                if c.callee.is_expr()
                    && (c.callee.as_expr().unwrap().is_ident_ref_to("_$")
                        || c.callee.as_expr().unwrap().is_ident_ref_to("$$")) =>
            {
                e
            }

            // is a render method call, keep as is
            Expr::Call(c)
                if c.callee.is_expr()
                    && RENDER_METHODS.iter().any(|m| c.callee.as_expr().unwrap().is_ident_ref_to(m)) =>
            {
                e.fold_children_with(self)
            }

            // convert redundant $()
            Expr::Call(c)
                if c.callee.is_expr() && (c.callee.as_expr().unwrap().is_ident_ref_to("always")) =>
            {
                Box::new(Expr::Call(self.fold_call_expr(c.clone())))
            }

            // default: wrap in always
            _ => {
                // check if body contains await -> is_async
                let mut async_checker = AsyncChecker { is_async: false };
                e.visit_with(&mut async_checker);
                let is_async = async_checker.is_async;

                let call_expr = Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                        always_fn_name.into(),
                        DUMMY_SP,
                        Default::default(),
                    )))),
                    args: vec![Expr::Arrow(ArrowExpr {
                        span: DUMMY_SP,
                        params: Take::dummy(),
                        body: Box::new(BlockStmtOrExpr::Expr(e)),
                        is_async,
                        is_generator: false,
                        type_params: Take::dummy(),
                        return_type: Take::dummy(),
                        ctxt: Default::default(),
                    })
                    .into()],
                    type_args: Take::dummy(),
                    ctxt: Default::default(),
                });

                // add await if async
                if is_async {
                    Box::new(Expr::Await(AwaitExpr {
                        span: DUMMY_SP,
                        arg: Box::new(call_expr)
                    }))
                }
                else {
                    Box::new(call_expr)
                }
            },
        }
    }

    fn transform_transferable_closure(arrow: &ArrowExpr, ctxt: SyntaxContext) -> ArrowExpr {
        // find all variables used in the arrow function body
        let mut collector = VariableCollector::new();

        // add arrow function params to collector variable_declarations
        for param in collect_params(&arrow.params, false) {
            if !collector.variable_declarations.contains(&param) {
                collector.variable_declarations.push(param);
            }
        }

        arrow.body.visit_children_with(&mut collector);

        let mut body_vec = vec![];

        // add use();
        if collector.used_variables.len() > 0 && !collector.has_custom_use {

            // add "silent-errors" string literal as first arg
            let args = vec![
                ExprOrSpread {
                    expr: Box::new(
                        Expr::Lit(Lit::Str(Str {
                            span: DUMMY_SP,
                            value: "silent-errors".into(),
                            raw: None,
                        })),
                    ),
                    spread: None,
                }
            ];
            let used_vars = collector
                .used_variables
                .iter()
                .map(|v| {
                    Expr::Ident(Ident::new(
                        v.0.clone(),
                        DUMMY_SP,
                        v.1,
                    ))
                })
                .map(|v| ExprOrSpread {
                    expr: Box::new(v),
                    spread: None,
                });

            let args = args.into_iter().chain(used_vars).collect();
            
            body_vec.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                        "use".into(),
                        DUMMY_SP,
                        ctxt,
                    )))),
                    args,
                    type_args: Take::dummy(),
                    ctxt,
                })),
            }))
        }

        // add original body
        match &*arrow.body {
            BlockStmtOrExpr::BlockStmt(b) => {
                for stmt in b.stmts.iter() {
                    body_vec.push(stmt.clone());
                }
            }
            BlockStmtOrExpr::Expr(e) => {
                // return + orignal expr
                body_vec.push(Stmt::Return(ReturnStmt {
                    span: DUMMY_SP,
                    arg: Some(Box::new(*e.clone())),
                }));
            }
        }

        ArrowExpr {
            span: arrow.span,
            ctxt: arrow.ctxt,
            params: arrow.params.clone(),
            // add use(); followed by original body
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                ctxt: arrow.ctxt,
                stmts: body_vec,
            })),
            is_async: arrow.is_async,
            is_generator: arrow.is_generator,
            type_params: arrow.type_params.clone(),
            return_type: arrow.return_type.clone(),
        }
    }

    fn transform_transferable_call_expr(call: &CallExpr) -> CallExpr {
        let arg = TransformVisitor::get_first_arg(call);

        match arg.unwrap_parens() {
            // is arrow function callback
            Expr::Arrow(a) => CallExpr {
                span: call.span,
                callee: call.callee.clone(),
                args: vec![Box::new(Expr::Arrow(
                    TransformVisitor::transform_transferable_closure(a, call.ctxt),
                ))
                .into()],
                type_args: call.type_args.clone(),
                ctxt: call.ctxt,
            },
            // TODO: other functions
            _ => call.clone(),
        }
    }

    fn get_first_arg(call: &CallExpr) -> Box<Expr> {
        call.clone()
            .args
            .into_iter()
            .nth(0)
            .unwrap_or(ExprOrSpread {
                expr: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
                spread: None,
            })
            .expr
    }
}

impl Fold for TransformVisitor {
    fn fold_call_expr(&mut self, call: CallExpr) -> CallExpr {
        return match &call.callee {
            Callee::Expr(e) => {
                let arg = TransformVisitor::get_first_arg(&call);

                return match e.unwrap_parens() {
                    Expr::Ident(i) if i.sym.eq_ignore_ascii_case("always") => {
                        return match arg.unwrap_parens() {
                            // constant - wrap in $$ ()
                            Expr::Lit(_) | Expr::JSXElement(_) | Expr::Ident(_) | Expr::This(_) => CallExpr {
                                span: DUMMY_SP,
                                callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                                    "$".into(),
                                    DUMMY_SP,
                                    call.ctxt,
                                )))),
                                args: vec![arg.fold_with(self).into()],
                                type_args: Take::dummy(),
                                ctxt: call.ctxt,
                            },

                            // callback wrapper, no transformation needed
                            Expr::Fn(_) => call,

                            // default: wrap in always
                            _ => {
                                let reactive = self.transform_expr_reactive(arg.clone(), "always");
                                match reactive.unwrap_parens() {
                                    Expr::Call(c) => c.clone(),
                                    // transform_expr_reactive returns a CallExpr in all cases except for Expr::Arrow(_) | Expr::Fn
                                    _ => CallExpr {
                                        span: DUMMY_SP,
                                        callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                                            "always".into(),
                                            DUMMY_SP,
                                            call.ctxt,
                                        )))),
                                        args: vec![
                                            match arg.unwrap_parens() {
                                                Expr::Arrow(_) | Expr::Fn(_) => arg.into(),
                                                _ => Expr::Arrow(ArrowExpr {
                                                    span: DUMMY_SP,
                                                    params: Take::dummy(),
                                                    body: Box::new(BlockStmtOrExpr::Expr(arg)),
                                                    is_async: false,
                                                    is_generator: false,
                                                    type_params: Take::dummy(),
                                                    return_type: Take::dummy(),
                                                    ctxt: call.ctxt,
                                                })
                                                .into(),
                                            }
                                        ],
                                        type_args: Take::dummy(),
                                        ctxt: call.ctxt,
                                    }
                                }
                            }
                        };
                    }

                    Expr::Ident(i) if i.sym.eq_ignore_ascii_case("run") => {
                        // add "use()" to run (()=>{})
                        return TransformVisitor::transform_transferable_call_expr(&call);
                    }

                    Expr::Ident(i) if i.sym.eq_ignore_ascii_case("renderFrontend") => {
                        // add "use()" to renderFrontend (()=>{})
                        return TransformVisitor::transform_transferable_call_expr(&call);
                    }

                    _ => call.fold_children_with(self),
                };
            }
            _ => call.fold_children_with(self),
        };

        // if n.callee.is_expr() && n.callee.expect_expr().expect_ident().sym.eq_ignore_ascii_case("$") {
        //         return CallExpr {
        //             span: DUMMY_SP,
        //             callee: Callee::Expr(Box::new(Expr::Ident(Ident::new("_$".into(), DUMMY_SP)))),
        //             args: vec![Expr::Arrow(ArrowExpr {
        //                 span: DUMMY_SP,
        //                 params: Take::dummy(),
        //                 body: Box::new(BlockStmtOrExpr::Expr(n.args.into_iter().nth(0).expect("invalid $()").expr)),
        //                 is_async: false,
        //                 is_generator: false,
        //                 type_params: Take::dummy(),
        //                 return_type: Take::dummy(),
        //             }).into()],
        //             type_args: Take::dummy(),
        //         }
        // }

        // return n
    }

    fn fold_jsx_attr(&mut self, node: JSXAttr) -> JSXAttr {
        // if attribute ends with :frontend, transform_transferable_call_expr
        match node.name.clone() {
            JSXAttrName::JSXNamespacedName(name)
                if name.name.sym.eq_ignore_ascii_case("frontend") =>
            {
                match node.value.clone() {
                    Some(JSXAttrValue::JSXExprContainer(c)) => {
                        match c.expr.clone() {
                            JSXExpr::Expr(e) => match e.unwrap_parens() {
                                Expr::Arrow(a) => return JSXAttr {
                                    span: node.span,
                                    name: node.name.clone(),
                                    value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                                        span: DUMMY_SP,
                                        expr: JSXExpr::Expr(Box::new(Expr::Arrow(
                                            TransformVisitor::transform_transferable_closure(
                                                &a, a.ctxt,
                                            ),
                                        ))),
                                    })),
                                },
                                Expr::Call(c) => return JSXAttr {
                                    span: node.span,
                                    name: node.name.clone(),
                                    value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                                        span: DUMMY_SP,
                                        expr: JSXExpr::Expr(Box::new(Expr::Call(
                                            TransformVisitor::transform_transferable_call_expr(&c),
                                        ))),
                                    })),
                                },
                                _ => JSXAttr {
                                    span: node.span,
                                    name: node.name.clone(),
                                    value: Some(JSXAttrValue::JSXExprContainer(
                                        self.fold_jsx_expr_container(c),
                                    )),
                                },
                            },
                            _ => node,
                        }
                    }
                    _ => node,
                }
            }
            _ => match node.value.clone() {
                Some(JSXAttrValue::JSXExprContainer(c)) => JSXAttr {
                    span: node.span,
                    name: node.name.clone(),
                    value: Some(JSXAttrValue::JSXExprContainer(
                        self.fold_jsx_expr_container(c),
                    )),
                },
                _ => node,
            },
        }
    }

    fn fold_jsx_element_child(&mut self, child: JSXElementChild) -> JSXElementChild {
        match child {
            JSXElementChild::JSXExprContainer(c) => JSXElementChild::JSXExprContainer(
                self.fold_jsx_expr_container(c),
            ),
            JSXElementChild::JSXSpreadChild(c) => JSXElementChild::JSXSpreadChild(
                JSXSpreadChild {
                    span: DUMMY_SP,
                    expr: self.transform_expr_reactive(c.expr, "_$")
                }
            ),
            JSXElementChild::JSXElement(e) => JSXElementChild::JSXElement(
                Box::new(JSXElement {
                    span: DUMMY_SP,
                    opening: e.opening.fold_with(self),
                    children: self.fold_jsx_element_childs(e.children),
                    closing: e.closing,
                })
            ),
            _ => child
        }
    }

    fn fold_jsx_element_childs(&mut self, node: Vec<JSXElementChild>) -> Vec<JSXElementChild> {
        let mut previous_was_hash_text = false;

        node.into_iter()
            .map(|mut child| {

                if previous_was_hash_text {
                    // add a space after text containing "#"
                    match &child {
                        JSXElementChild::JSXExprContainer(_) => {
                            previous_was_hash_text = false;
                            return child;
                        },
                        _ => (),
                    };
                }

                // remember if element is text containing "#"
                previous_was_hash_text = match &child {
                    JSXElementChild::JSXText(t) => t.value.trim_end().ends_with("#static"),
                    _ => false,
                };

                // remove the hash at the end of the text
                if previous_was_hash_text {
                    child = match child {
                        JSXElementChild::JSXText(t) => {
                            let trimmed_val = t.value.trim_end();
                            let val: Atom = trimmed_val[..trimmed_val.len() - 7].into();
                            
                            JSXElementChild::JSXText(JSXText {
                                span: t.span,
                                value: val.clone(),
                                raw: val,
                            })
                        },
                        _ => child,
                    };
                }

                child.fold_with(self)
            })
            .collect()
    }

    fn fold_jsx_expr_container(&mut self, n: JSXExprContainer) -> JSXExprContainer {
        JSXExprContainer {
            span: DUMMY_SP,
            expr: (match n.expr {
                JSXExpr::Expr(e) => JSXExpr::Expr(self.transform_expr_reactive(e, "_$")),
                JSXExpr::JSXEmptyExpr(_) => JSXExpr::JSXEmptyExpr(JSXEmptyExpr { span: DUMMY_SP }),
            }),
        }
    }
}
