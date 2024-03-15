#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_arena;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_attr;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_lexer;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_parse;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;

use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, ExprKind,
};
use rustc_lint::LateLintPass;
use rustc_span::{Span, Symbol};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Known problems
    /// Remove if none.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub MI_DETECTOR,
    Warn,
    "Amazing, you are using gas_left()!"
}

struct estaLaFuncion {
    is_gas_left: bool,
    is_gas_left_span: Vec<Option<Span>>,
}

impl<'tcx> Visitor<'tcx> for estaLaFuncion{
    fn visit_expr(&mut self, expr: &'tcx Expr<'_>){

        if let ExprKind::MethodCall(path_segment, _, _, _) = &expr.kind {
            if path_segment.ident.name == Symbol::intern("gas_left") {
                self.is_gas_left = true;
                self.is_gas_left_span.push(Some(expr.span));
            }
        }
        walk_expr(self, expr);
    }
}

impl<'tcx> LateLintPass<'tcx> for MiDetector {
    fn check_fn(
        &mut self,
        cx: &rustc_lint::LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: rustc_span::Span,
        _: rustc_hir::def_id::LocalDefId,
    ) {
        let mut visitor = estaLaFuncion{
            is_gas_left: false,
            is_gas_left_span: Vec::new(),
        };

        walk_expr(&mut visitor, body.value);

        if visitor.is_gas_left{
            visitor.is_gas_left_span.iter().for_each(|span| {
                
                if let Some(span) = span {
                    
                    clippy_utils::diagnostics::span_lint(
                        cx,
                        MI_DETECTOR,
                        *span,
                        "Amazing, you are using gas_left()!"

                    );
                }
            });
        }
    }
   
}



#[test]
fn ui() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
    );
}
