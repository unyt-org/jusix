use swc_core::{
    atoms::Atom,
    common::{util::take::Take, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            ArrowExpr, AwaitExpr, BlockStmt, BlockStmtOrExpr, CallExpr, Callee, Expr, ExprOrSpread, ExprStmt, FnDecl, Ident, JSXAttr, JSXAttrName, JSXAttrValue, JSXElement, JSXElementChild, JSXElementName, JSXEmptyExpr, JSXExpr, JSXExprContainer, JSXSpreadChild, Lit, MemberProp, Null, ObjectPatProp, Pat, ReturnStmt, Stmt, Str, VarDecl
        },
        visit::{Fold, FoldWith, Visit, VisitMutWith, VisitWith},
    },
};

struct VariableCollector {
    used_variables: Vec<String>,
    variable_declarations: Vec<String>,
}

impl VariableCollector {
    fn new() -> Self {
        VariableCollector {
            used_variables: Vec::new(),
            variable_declarations: Vec::new(),
        }
    }
}

impl Visit for VariableCollector {

    fn visit_jsx_element_name(&mut self, _name: &JSXElementName) {
        // ignore jsx name as identifier
    }

    fn visit_ident(&mut self, ident: &Ident) {
        // add variable to list if not already present and not
        // in variable_declarations
        let name = ident.sym.to_string();
        if !self.used_variables.contains(&name) &&
            !self.variable_declarations.contains(&name) &&
            !GLOBAL_THIS_ALIASES.contains(&name.as_str()) {
            self.used_variables.push(name);
        }
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl) {
        for decl in &var_decl.decls {
            // add variable to list if not already present
            match &decl.name {
                Pat::Ident(i) => {
                    let name = i.sym.to_string();
                    if !self.variable_declarations.contains(&name) {
                        self.variable_declarations.push(name);
                    }
                }
                Pat::Object(o) => {
                    for prop in &o.props {
                        match prop {
                            ObjectPatProp::KeyValue(kv) => {
                                match *kv.value.clone() {
                                    Pat::Ident(i) => {
                                        let name = i.sym.to_string();
                                        if !self.variable_declarations.contains(&name) {
                                            self.variable_declarations.push(name);
                                        }
                                    }
                                    _ => {}
                                    
                                }
                            }
                            ObjectPatProp::Assign(a) => {
                                let name = a.key.sym.to_string();
                                if !self.variable_declarations.contains(&name) {
                                    self.variable_declarations.push(name);
                                }
                            }
                            ObjectPatProp::Rest(r) => {
                                if let Pat::Ident(i) = &*r.arg {
                                    let name = i.sym.to_string();
                                    if !self.variable_declarations.contains(&name) {
                                        self.variable_declarations.push(name);
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
                                let name = i.sym.to_string();
                                if !self.variable_declarations.contains(&name) {
                                    self.variable_declarations.push(name);
                                }
                            }
                            Some(Pat::Array(a)) => {
                                for elem in &a.elems {
                                    match elem {
                                        Some(Pat::Ident(i)) => {
                                            let name = i.sym.to_string();
                                            if !self.variable_declarations.contains(&name) {
                                                self.variable_declarations.push(name);
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
        // Visit the function body
        fn_decl.function.body.visit_with(self);
    }

    fn visit_block_stmt(&mut self, block_stmt: &BlockStmt) {
        for stmt in &block_stmt.stmts {
            stmt.visit_with(self);
        }
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
    fn visit_fn_expr(&mut self, _node: &swc_core::ecma::ast::FnExpr) {}
}


const GLOBAL_THIS_ALIASES: [&'static str; 3] = [
    "globalThis",
    "self",
    "window",
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
            Expr::Lit(_) | Expr::JSXElement(_) | Expr::Ident(_) => e,

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
        arrow.body.visit_with(&mut collector);

        let mut body_vec = vec![];

        // add use();
        if collector.used_variables.len() > 0 {
            body_vec.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                        "use".into(),
                        DUMMY_SP,
                        ctxt,
                    )))),
                    args: collector
                        .used_variables
                        .iter()
                        // ignore "use" variable
                        .filter(|v| !(*v == "use"))
                        .map(|v| {
                            Expr::Lit(Lit::Str(Str {
                                span: DUMMY_SP,
                                value: Atom::from(v.clone()),
                                raw: Some(Atom::from(v.clone())),
                            }))
                        })
                        .map(|v| v.into())
                        .collect(),
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
                            Expr::Lit(_) | Expr::JSXElement(_) | Expr::Ident(_) => CallExpr {
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
        node.into_iter()
            .map(|child| child.fold_with(self))
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
