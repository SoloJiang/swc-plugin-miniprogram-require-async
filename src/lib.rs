use swc_core::ecma::{
    ast::{Program, CallExpr, Callee, Expr, Ident},
    visit::{as_folder, noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

pub fn transform_dynamic_import() -> impl VisitMut {
    TransformVisitor
}

impl VisitMut for TransformVisitor {
    noop_visit_mut_type!();
    fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
        call_expr.visit_mut_children_with(self);

        match &mut call_expr.callee {
            Callee::Import(_) => {
                let identifier = Ident::new("require.async".into(), Default::default());
                call_expr.callee = Callee::Expr(Box::new(Expr::Ident(identifier)));
            }
            Callee::Super(_) => {}
            Callee::Expr(_) => {}
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
    program.fold_with(&mut as_folder(TransformVisitor))
}
