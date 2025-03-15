use swc_core::ecma::visit::VisitWith;
use swc_core::ecma::{ast::Program, transforms::testing::test, visit::FoldWith};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_ecma_parser::{EsSyntax, Syntax, TsSyntax};
use visitor::TransformVisitor;

pub mod visitor;
mod reactive_positions_extractor;

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    // extract the first comment node from the program to get reactive positions metadata
    let mut positions_visitor = reactive_positions_extractor::ReactivePositionsVisitor::default();
    program.visit_children_with(&mut positions_visitor);
    program.fold_with(&mut TransformVisitor::with_reactive_positions(positions_visitor.positions))
}

// Test reactive position extraction with ReactivePositionsVisitor
#[cfg(test)]
mod test {
    use swc_core::common::errors::{ColorConfig, Handler};
    use swc_core::common::{FileName, SourceMap, DUMMY_SP};
    use swc_core::ecma::ast::{Module, Program};
    use swc_core::ecma::visit::{FoldWith, VisitWith};
    use swc_ecma_codegen::to_code_default;
    use swc_ecma_parser::lexer::Lexer;
    use swc_ecma_parser::{Capturing, Parser, StringInput, Syntax, TsSyntax};
    use crate::visitor::TransformVisitor;

    use super::reactive_positions_extractor;
    use swc_common::{sync::Lrc};

    fn source_to_module(src: String) -> Module {
        let cm: Lrc<SourceMap> = Default::default();
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

        
        let fm = cm.new_source_file(
            FileName::Custom("test.ts".into()).into(),
            src.into(),
        );
    
        let lexer = Lexer::new(
            Syntax::Typescript(TsSyntax {
                tsx: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );
    
        let capturing = Capturing::new(lexer);
    
        let mut parser = Parser::new_from(capturing);
    
        for e in parser.take_errors() {
            e.into_diagnostic(&handler).emit();
        }
    
        parser
            .parse_typescript_module()
            .map_err(|e| e.into_diagnostic(&handler).emit())
            .expect("Failed to parse module.")
    }

    fn module_to_source(module: &Module) -> String {
        let cm: Lrc<SourceMap> = Default::default();
        to_code_default(cm.clone(), None, &module)
    }

    #[test]
    fn reactive_position_extraction() {
        // create mock program with a comment node
        let program = source_to_module("const __UIX_REACTIVE_POSITIONS=[10,42,100000];\nimport 'x.ts';\nconst test = 10;".to_string());
        let mut positions_visitor = reactive_positions_extractor::ReactivePositionsVisitor::default();

        program.visit_children_with(&mut positions_visitor);

        assert_eq!(positions_visitor.positions, Some(vec![10,42,100000]));
    }

    #[test]
    fn reactive_position_extraction_none() {
        // create mock program with a comment node
        let program = source_to_module("import 'x.ts';\nconst test = 10;".to_string());
        let mut positions_visitor = reactive_positions_extractor::ReactivePositionsVisitor::default();

        program.visit_children_with(&mut positions_visitor);

        assert_eq!(positions_visitor.positions, None);
    }

    #[test]
    fn reactive_position_extraction_empty() {
        // create mock program with a comment node
        let program = source_to_module("const __UIX_REACTIVE_POSITIONS=[];\nimport 'x.ts';\nconst test = 10;".to_string());
        let mut positions_visitor = reactive_positions_extractor::ReactivePositionsVisitor::default();

        program.visit_children_with(&mut positions_visitor);

        assert_eq!(positions_visitor.positions, Some(Vec::<u32>::new()));
    }

    #[test]
    fn reactive_attribute_no_positions() {
        // create mock program with a comment node
        let program = source_to_module("export default <Test a={ x + 1 } />;".to_string());
        let mut positions_visitor = reactive_positions_extractor::ReactivePositionsVisitor::default();

        program.visit_children_with(&mut positions_visitor);
        assert_eq!(positions_visitor.positions, None);

        let program = program.fold_with(&mut TransformVisitor::with_reactive_positions(positions_visitor.positions));
        let source_code = module_to_source(&program);

        assert_eq!(source_code, "export default <Test a={_$(()=>x + 1)}/>;\n");
    }

    #[test]
    fn non_reactive_attribute() {
        // create mock program with a comment node
        let program = source_to_module("const __UIX_REACTIVE_POSITIONS=[];export default <Test a={ x + 1 } />;".to_string());
        let mut positions_visitor = reactive_positions_extractor::ReactivePositionsVisitor::default();

        program.visit_children_with(&mut positions_visitor);

        assert_eq!(positions_visitor.positions, Some(Vec::<u32>::new()));

        let program = program.fold_with(&mut TransformVisitor::with_reactive_positions(positions_visitor.positions));
        let source_code = module_to_source(&program);

        assert_eq!(source_code, "const __UIX_REACTIVE_POSITIONS = [];\nexport default <Test a={x + 1}/>;\n");
    }

    #[test]
    fn reactive_attribute() {
        // create mock program with a comment node
        let program = source_to_module("const __UIX_REACTIVE_POSITIONS=[0];export default <Test a={ x + 1 } />;".to_string());
        let mut positions_visitor = reactive_positions_extractor::ReactivePositionsVisitor::default();

        program.visit_children_with(&mut positions_visitor);

        assert_eq!(positions_visitor.positions, Some(vec![0]));

        let program = program.fold_with(&mut TransformVisitor::with_reactive_positions(positions_visitor.positions));
        let source_code = module_to_source(&program);

        assert_eq!(source_code, "const __UIX_REACTIVE_POSITIONS = [\n    0\n];\nexport default <Test a={_$(()=>x + 1)}/>;\n");
    }

    #[test]
    fn reactive_lit_attribute() {
        // create mock program with a comment node
        let program = source_to_module("const __UIX_REACTIVE_POSITIONS=[0];export default <Test a={ 42 } />;".to_string());
        let mut positions_visitor = reactive_positions_extractor::ReactivePositionsVisitor::default();

        program.visit_children_with(&mut positions_visitor);

        assert_eq!(positions_visitor.positions, Some(vec![0]));

        let program = program.fold_with(&mut TransformVisitor::with_reactive_positions(positions_visitor.positions));
        let source_code = module_to_source(&program);

        assert_eq!(source_code, "const __UIX_REACTIVE_POSITIONS = [\n    0\n];\nexport default <Test a={_$(()=>42)}/>;\n");
    }

    #[test]
    fn reactive_lit_string_attribute() {
        // create mock program with a comment node
        let program = source_to_module("const __UIX_REACTIVE_POSITIONS=[0];export default <Test a=\"xyz\" />;".to_string());
        let mut positions_visitor = reactive_positions_extractor::ReactivePositionsVisitor::default();

        program.visit_children_with(&mut positions_visitor);

        assert_eq!(positions_visitor.positions, Some(vec![0]));

        let program = program.fold_with(&mut TransformVisitor::with_reactive_positions(positions_visitor.positions));
        let source_code = module_to_source(&program);

        assert_eq!(source_code, "const __UIX_REACTIVE_POSITIONS = [\n    0\n];\nexport default <Test a={_$(()=>\"xyz\")}/>;\n");
    }

    #[test]
    fn reactive_lit_string_attribute_namespaced() {
        // create mock program with a comment node
        let program = source_to_module("const __UIX_REACTIVE_POSITIONS=[0];export default <Test a:frontend=\"xyz\" />;".to_string());
        let mut positions_visitor = reactive_positions_extractor::ReactivePositionsVisitor::default();

        program.visit_children_with(&mut positions_visitor);

        assert_eq!(positions_visitor.positions, Some(vec![0]));

        let program = program.fold_with(&mut TransformVisitor::with_reactive_positions(positions_visitor.positions));
        let source_code = module_to_source(&program);

        assert_eq!(source_code, "const __UIX_REACTIVE_POSITIONS = [\n    0\n];\nexport default <Test a:frontend={_$(()=>\"xyz\")}/>;\n");
    }

    #[test]
    fn reactive_lit_string_attribute_boolean() {
        // create mock program with a comment node
        let program = source_to_module("const __UIX_REACTIVE_POSITIONS=[0];export default <Test enabled />;".to_string());
        let mut positions_visitor = reactive_positions_extractor::ReactivePositionsVisitor::default();

        program.visit_children_with(&mut positions_visitor);

        assert_eq!(positions_visitor.positions, Some(vec![0]));

        let program = program.fold_with(&mut TransformVisitor::with_reactive_positions(positions_visitor.positions));
        let source_code = module_to_source(&program);

        assert_eq!(source_code, "const __UIX_REACTIVE_POSITIONS = [\n    0\n];\nexport default <Test enabled={_$(()=>true)}/>;\n");
    }
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
test!(
    Default::default(),
    |_| TransformVisitor::default(),
    t1,
    r#"const x = always(10)"#
);

test!(
    Default::default(),
    |_| TransformVisitor::default(),
    t2,
    r#"const y = always(y * 2)"#
);

test!(
    Default::default(),
    |_| TransformVisitor::default(),
    t3,
    r#"run((a) => {
        console.log(a, x + y);
        return x + 1;
    })"#
);

test!(
    Default::default(),
    |_| TransformVisitor::default(),
    t4,
    r#"run(() => x + 1)"#
);

test!(
    Default::default(),
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
    t6,
    r#"<button onclick:frontend={() => console.log(x)} />"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t7,
    r#"<button value={x+1} />"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t8,
    r#"<button value:frontend={x+1} />"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t9,
    r#"<button value:frontend={always(() => x+1)} />"#
);

test!(
    Default::default(),
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
    t11,
    r#"<div>{ x + 1 }</div>"#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
    t14,
    r#"<div>{x.title}</div>"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t15,
    r#"<input value={x.name}/>"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t16,
    r#"<input value={x.$.name} id={x.$$.name}/>"#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t17,
    r#"<input value={x['äü']}/>"#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
    t20,
    r#"<input value={x[0]}/>"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t21,
    r#"<input value={x.y.z[0]}/>"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t22,
    r#"const x = arr.map(a => a*2)"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t23,
    r#"const x = always(arr.map(a => a*2))"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t24,
    r#"const x = always(() => x + 1)"#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t25,
    r#"const x = always(x.$.y)"#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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
    |_| TransformVisitor::default(),
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


test!(
    Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t53,
    r#"
    class X {
        method() {
            return <button 
                onclick:frontend={() => this.handleSettings(data)}>
                Apply settings
            </button>
        }
    }
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t54,
    r#"<button onclick:frontend={function () {console.log(x)}} />"#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t55,
    r#"
    run(function xy () {
        console.log(x);
    })
    "#
);



test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t56,
    r#"<div>
        {
            array
                .filter(item => item > 0)
                .map(item => <span>{item}</span>)
        }
    </div>
    "#
);



test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t57,
    r#"
    const test = always(() => x + 1, {allowStatic: true});
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::with_reactive_positions(Some(vec![0])),
    t58,
    r#"
    export default <Test a={ x + 1 } />;
    "#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::with_reactive_positions(Some(vec![1])),
    t59,
    r#"
    export default <Test a={ x + 1 } />;
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::with_reactive_positions(Some(vec![1])),
    t60,
    r#"
    export default <Test a={ x + 1 } b = { y + 1 } />;
    "#
);

test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::with_reactive_positions(Some(vec![1])),
    t61,
    r#"
    export default <Test a={ x + 1 } boolean />;
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::with_reactive_positions(Some(vec![4,7,9])),
    t62,
    r#"
    const x = <div a0="5" a1>
        <span a2={b()}>Test</span>
    </div>;
    export default <Test a3 a4={ makeReactive() } a5 = { <div a6={x}>Test</div> } >
        <span>{x}</span>
        <span a7={reactiveVar}>
            Test
            <div a8={x + 1}>Test</div>
        </span>
        <div a9>Test</div>
    </Test>;
    "#
);


test!(
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    },),
    |_| TransformVisitor::default(),
    t63,
    r#"
    export default <Test a={ x +1 } inner={ <div b={ x + 1 }>Test</div> }>
        Content
    </Test>;
    "#
);
