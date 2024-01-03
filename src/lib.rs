use swc_core::ecma::{
    ast::Program,
    transforms::testing::test,
    visit::FoldWith,
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub mod visitor;
use visitor::TransformVisitor;

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