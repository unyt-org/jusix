use swc_core::{ecma::{
    ast::{Program, Ident, ArrowExpr, JSXEmptyExpr, JSXExpr, JSXExprContainer, BlockStmtOrExpr, Expr, CallExpr, Callee, ExprOrSpread, Lit, JSXText, Null},
    transforms::testing::test,
    visit::{FoldWith, Fold},
}, common::{Span, util::take::Take, DUMMY_SP}};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

impl TransformVisitor {
    // wraps in expression in always() if needed
    fn transform_expr_reactive(&mut self, e: Box<Expr>) -> Box<Expr> {

        match e.unwrap_parens() {
            // keep single literal values
            Expr::Lit(_) |
            Expr::JSXElement(_) |
            Expr::Ident(_) => e,

            // has a $.x property, don't add always
            Expr::Member(m) if 
                m.obj.is_member() && 
                (
                    m.obj.as_member().unwrap().prop.is_ident_with("$") ||
                    m.obj.as_member().unwrap().prop.is_ident_with("$$") 
                )
                => e,
                

            // already has an always() or $$() wrapper
            Expr::Call(c) if 
                c.callee.is_expr() && (
                    c.callee.as_expr().unwrap().is_ident_ref_to("always") ||
                    c.callee.as_expr().unwrap().is_ident_ref_to("$$")
                )
                => e,
            
            // convert redundant $()
            Expr::Call(c) if 
                c.callee.is_expr() && (
                    c.callee.as_expr().unwrap().is_ident_ref_to("$")
                )
                => Box::new(Expr::Call(self.fold_call_expr(c.clone()))),
    
            // default: wrap in always
            _ => Box::new(
                Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    callee: Callee::Expr(Box::new(Expr::Ident(Ident::new("always".into(), DUMMY_SP)))),
                    args: vec![Expr::Arrow(ArrowExpr {
                        span: DUMMY_SP,
                        params: Take::dummy(),
                        body: Box::new(BlockStmtOrExpr::Expr(e)),
                        is_async: false,
                        is_generator: false,
                        type_params: Take::dummy(),
                        return_type: Take::dummy(),
                    }).into()],
                    type_args: Take::dummy(),
                })
            
        )
        }

        
    }
}
 
impl Fold for TransformVisitor {

    fn fold_call_expr(&mut self, n: CallExpr) -> CallExpr {

        return match &n.callee {
            Callee::Expr(e) => {
                return match e.unwrap_parens() {
                    Expr::Ident(i) if i.sym.eq_ignore_ascii_case("$") => {

                        let arg = n.args.into_iter().nth(0).unwrap_or(ExprOrSpread {expr:Box::new(Expr::Lit(Lit::Null(Null {span:DUMMY_SP}))), spread:None}).expr;

                        return match arg.unwrap_parens() {
                            // $$ ()
                            Expr::Lit(_) | 
                            Expr::JSXElement(_) | 
                            Expr::Ident(_) => CallExpr {
                                span: DUMMY_SP,
                                callee: Callee::Expr(Box::new(Expr::Ident(Ident::new("$$".into(), DUMMY_SP)))),
                                args: vec![arg.into()],
                                type_args: Take::dummy(),
                            },

                            // default: wrap in always
                            _ => CallExpr {
                                span: DUMMY_SP,
                                callee: Callee::Expr(Box::new(Expr::Ident(Ident::new("always".into(), DUMMY_SP)))),
                                args: vec![Expr::Arrow(ArrowExpr {
                                    span: DUMMY_SP,
                                    params: Take::dummy(),
                                    body: Box::new(BlockStmtOrExpr::Expr(arg)),
                                    is_async: false,
                                    is_generator: false,
                                    type_params: Take::dummy(),
                                    return_type: Take::dummy(),
                                }).into()],
                                type_args: Take::dummy(),
                            }
                        }
                    }
                    _ => n
                };
            },
            _ => n
        };

        // if n.callee.is_expr() && n.callee.expect_expr().expect_ident().sym.eq_ignore_ascii_case("$") {
        //         return CallExpr {
        //             span: DUMMY_SP,
        //             callee: Callee::Expr(Box::new(Expr::Ident(Ident::new("always".into(), DUMMY_SP)))),
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

    fn fold_jsx_expr_container(&mut self, n: JSXExprContainer) -> JSXExprContainer {

        JSXExprContainer {
            span: Span::dummy_with_cmt(),
            expr: (

                match n.expr {
                    JSXExpr::Expr(e) => JSXExpr::Expr(self.transform_expr_reactive(e)),
                    JSXExpr::JSXEmptyExpr(_) => JSXExpr::JSXEmptyExpr(JSXEmptyExpr { span: DUMMY_SP })
                }

            )
        }
    }
        
}
/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut TransformVisitor)
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
test!(
    Default::default(),
    |_| TransformVisitor,
    t1,
    r#"const x = $(10)"#
);

test!(
    Default::default(),
    |_| TransformVisitor,
    t2,
    r#"const y = $(y * 2)"#
);