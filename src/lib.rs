use swc_core::ecma::{ast::Program, transforms::testing::test, visit::FoldWith};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_ecma_parser::{EsSyntax, Syntax, TsSyntax};
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
    r#"const x = always(10)"#
);

test!(
    Default::default(),
    |_| TransformVisitor,
    t2,
    r#"const y = always(y * 2)"#
);

test!(
    Default::default(),
    |_| TransformVisitor,
    t3,
    r#"run((a) => {
        console.log(a, x + y);
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
    r#"const x = always(arr.map(a => a*2))"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t24,
    r#"const x = always(() => x + 1)"#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t25,
    r#"const x = always(x.$.y)"#
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
    const y = always(<div>{x+1}</div>);
    const z = always(42);
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
    const x = always([
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
    const x = always([
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
        const y = always(42)
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
        const y = always(42)
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
        const y = always(42)
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
        const y = always(42 + x);
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
    <div>
        <span>{x + 1}</span>
        <span>{x}</span>
        <span>{1}</span>
        <span>inline&nbsp;text</span>
    </div>
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t36,
    r#"
    let x = 10;
    run(() => {
        console.log(x + 1);
    })
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t37,
    r#"
    <input 
		type="button"
		value="Hello"
		onclick:frontend={async () => {
			const x = <BaseComponent title="x" color="red"/>;
            const y = 10;
            const { console } = globalThis; 
			console.log(x, y);
			globalThis.alert('feef')
			alert("Hello!");
		}}
	/>
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t38,
    r#"
    <div>
        {
            renderFrontend(
                () => <div>
                    <MyComponent />
                    4 + {x} = { x + 4 }
                </div>,
                "Loading......"
            )
        }
        {
            renderBackend(
                <div>{ x + 1 }</div>
            )
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
    t39,
    r#"
    <div>
        {
            await x(y)
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
    t40,
    r#"
    <div>
        {
            (async () => await x(y))()
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
    t41,
    r#"
    export default {
        '/test': async () => <Example user={await getCurrentUser()}/>
    }
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t42,
    r#"
    renderFrontend(async () => {
        function blabla(fn1, fn2) {
            console.log(fn1, fn2, fn3);
        }
        console.log(fn1, blabla());
    })
    "#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t43,
    r#"
    renderFrontend(() => {
        const _fn = function([fn4]) {
            const fn5 = 5;
            function innerFn(fn7) {
                console.log(fn4, fn5, fn7, fn8);
            }
            console.log(fn4, fn5, fn6);
        };
    });
    "#
);



test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t44,
    r#"
    renderFrontend(() => {
        const handleKeydown = (e, [_1, _2], { b: _3 }, ..._4) => {
            let lol = 1;
            console.log(e, _1, _2, _3, _4, xxx);
        };

        console.log(lol);
    });
    "#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t45,
    r#"
    renderFrontend(() => {
        console.log(null, undefined, this, globalThis, window, true, false, NaN, Infinity, -Infinity);
    });
    "#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t46,
    r#"
    renderFrontend(() => {
        {
            const inner1 = 1;
            const inner2 = 2;
            console.log(inner1, inner2, outer1);
        }
        console.log(inner1);
    });
    "#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t47,
    r#"
    renderFrontend(() => {
        try {} 
        catch (e1) {
            console.log(e1);
        }

        try {} 
        catch (e2) {
            console.log(e2);
        }
        console.log(e2);
    });
    "#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t48,
    r#"
    class A {
        test() {
            return <div>
                <input value={x.value} />
                <input value={this.value} />
                <input value={super.value} />
                <input value={a['x'].value} />
            </div>
        }
    }
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t49,
    r#"
    call(function ({title, icon, children}) {
        return <div class="card">
            <span title="Dialog schließen" onclick:frontend={()=>use(this) && this.closeDialog()} class="desktop-close"><i class="fa-solid fa-xmark"/></span>
            <h1>
                <span title="Zurück" onclick:frontend={()=> this.closeDialog()} class="mobile-back"><i class="fa-solid fa-chevron-left"/></span>
                { icon && <i class={`fas ${icon}`} style="margin-right:10px"/>}
                {title}
            </h1>
            {...children}
            {...(children)}
            {...[1,2,3]}
            {...([1,2,3].map((item) => <span>{item}</span>))}
        </div>
    });
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t50,
    r#"
    <div>
        <div>{x.y}</div>
        <div>#static{x.y}</div>
        <div>{x()}</div>
        <div>#static{x()}</div>
        <div>{x+1}</div>
        <div>#static{x+1}</div>

        <div>#static {x+1}</div>
        <div>#static 
        {x+1}
        </div>
        <div>#staticxy
        {x+1}
        </div>
        <div>#static{x+1}{x+2}</div>
    </div>
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t51,
    r#"
    <button 
        onclick:frontend={() => this.handleSettings()}>
        Apply settings
    </button>
    "#
);


test!(
    Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor,
    t52,
    r#"
    run(() => {
        const x1 = y as Z1;
        const x2 = y satisfies Z2;
        const x3: Z3 = y;
        const x4: {_x: Z4} = y;
        interface Interface {
            x: Z5;
        }

        enum Color {
            Red = 1,
            Green = 2,
            Blue = 3
        }

        console.log(Color.Red);

        class MyClass extends ParentClass {
            x: T;

            y = this.x

            method(methodParam: Z6) {
                super.method(methodParam);
                console.log(this);
                alert(1)
            }
            static staticMethod(staticMethodParam: Z7) {
                alert(2)
            }
            set setter(setterParam: Z8) {
                alert(3)
            }
            static set staticSetter(staticSetterParam: Z9) {
                alert(4)
            }

            #privateMethod(privateMethodParam: Z10) {
                alert(5)
            }
            static #privateStaticMethod(privateStaticMethodParam: Z11) {
                alert(6)
            }
        }

        class MyClass2<T> extends ParentClass2<T> {
            x: T;
        }

        console.log(new MyClass<W>());

        function f(a: A, b: B) {
            console.log(this);
            return a + b;
        }
    })
    "#
);