use swc_core::ecma::{ast::Program, transforms::testing::test, visit::FoldWith};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_ecma_parser::{EsSyntax, Syntax};
use visitor::TransformVisitor;

pub mod visitor;

// #[plugin_transform]
// pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
//     program.fold_with(&mut TransformVisitor)
// }

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

test!(
    Default::default(),
    |_| TransformVisitor,
    t3,
    r#"run(() => {
        console.log(x + y);
        return x + 1;
    })"#
);

test!(
    Default::default(),
    |_| TransformVisitor,
    t4,
    r#"run(() => x + 1)"#
);

test!(
    Default::default(),
    |_| TransformVisitor,
    t5,
    r#"run(() => {
        use(x);
        return x + y;
    })"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t6,
    r#"<button onclick:frontend={() => console.log(x)} />"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t7,
    r#"<button value={x+1} />"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t8,
    r#"<button value:frontend={x+1} />"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t9,
    r#"<button value:frontend={always(() => x+1)} />"#
);

test!(
    Default::default(),
    |_| TransformVisitor,
    t10,
    r#"normalCallback(() => {
        return x + y;
    })"#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t11,
    r#"<div>{ x + 1 }</div>"#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t12,
    r#"<div>
        <span>{ x + 1 }</span>
        <span>{ y + 1 }</span>
        <span>X + Y = { x + y }</span>
    </div>"#
);



test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t13,
    r#"<div>
        {
            x ? <span>{ x + 1 }</span> : <span>False</span>
        }
    </div>"#
);



test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t14,
    r#"<div>{x.title}</div>"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t15,
    r#"<input value={x.name}/>"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t16,
    r#"<input value={x.$.name} id={x.$$.name}/>"#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t17,
    r#"<input value={x['äü']}/>"#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t18,
    r#"<div>
        {
            array.map((item) => {
                return <span>{item}</span>
            })
    }
    </div>
    "#
);
