use swc_core::ecma::{ast::{AssignExpr, AssignPat, AssignTarget, BindingIdent, ExprOrSpread, Ident, Lit, Number, SimpleAssignTarget, VarDecl}, visit::{swc_ecma_ast, Visit}};

pub struct ReactivePositionsVisitor {
	pub positions: Option<Vec<u32>>,
}

impl Default for ReactivePositionsVisitor {
	fn default() -> Self {
		Self {
			positions: None,
		}
	}
}

impl ReactivePositionsVisitor {
	const UIX_REACTIVE_POSITIONS_VAR: &'static str = "__UIX_REACTIVE_POSITIONS";
}

impl Visit for ReactivePositionsVisitor {

	// TODO: optimize, don't visit all children when we found the variable
	
	fn visit_var_decl(&mut self, node: &VarDecl) {
        // check if variable name is UIX_REACTIVE_POSITIONS_VAR
		for decl in &node.decls {
			if let Some(ident) = &decl.name.as_ident() {
				if ident.sym == Self::UIX_REACTIVE_POSITIONS_VAR {
					// get assigned array value
					if let Some(init) = &decl.init {
						if init.is_array() {

							if self.positions.is_none() {
								self.positions = Some(Vec::new());
							}

							let array = init.as_array().unwrap();
							// get all elements of the array
							for elem in &array.elems {
								if let Some(expr_or_spread) = elem {
									if let Some(Lit::Num(Number{ value, ..})) = expr_or_spread.expr.as_lit() {
										self.positions.as_mut().unwrap().push(*value as u32);
									}
								}
							}
						}
					}
				}
			}
		}
		
    }
}