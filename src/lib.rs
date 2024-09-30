use swc_core::ecma::{ast::Program, transforms::testing::test, visit::FoldWith};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_ecma_parser::{EsSyntax, Syntax};
use visitor::TransformVisitor;

pub mod visitor;

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
            x ? 
                <span>{ x + 1 }</span> : 
                <span>False</span>
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


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t19,
    r#"<div>
        {
            array.map((item) => item)
        }
        {
            array.map((item) => item * x)
        }
         {
            array.map((item) => {
                return item * x;    
            })
        }
        {
            array.map((item) => {
                return <span>{item * x}</span>
            })
        }
        {
            array.map((item) => <div>{item * x}</div>)
        }
        {
            array.filter((item) => {
                return <span>{item * x}</span>
            })
        }
         {
            array.normalMethod((item) => {
                return <span>{item * 2}</span>
            })
        }
    </div>
    "#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t20,
    r#"<input value={x[0]}/>"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t21,
    r#"<input value={x.y.z[0]}/>"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t22,
    r#"const x = arr.map(a => a*2)"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t23,
    r#"const x = $(arr.map(a => a*2))"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t24,
    r#"const x = $(() => x + 1)"#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t25,
    r#"const x = $(x.$.y)"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t26,
    r#"
    const x = <div>{x+1}</div>;
    const x = $(<div>{x+1}</div>);
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t27,
    r#"
    const x = $([
        1,2,y+1
    ])
    "#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t28,
    r#"
    const x = $([
        1,2,3
    ])
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t29,
    r#"
    export default <div>
        Count is {count + 1}
    </div>;
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t30,
    r#"
    const x = <div>
		<input value={count + '2'}/>
        <div>{x + y}</div>
        <div>
            <div id={ x + y}>
                <span class="static">{ x + y }</span>
            </div>
        </div>
	</div>
    "#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t31,
    r#"
    function x () {
        const y = $(42)
    }
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t32,
    r#"
    call(function () {
        const y = $(42)
    })
    "#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t33,
    r#"
    () => {
        const y = $(42)
        const z = <div>{y+1}</div>
    }
    "#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t34,
    r#"
    template(() => {
        const y = $(42)
        const z = <div>{y+1}</div>
    })
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t35,
    r#"
    run(async () => {
        const { f, x: g, ...z } = await import("datex-core-legacy");
        const [v, ...w] = externalFn();
        let x = 10;
        console.log(x, f, g, z);
        f(1);
    })
    "#
);
