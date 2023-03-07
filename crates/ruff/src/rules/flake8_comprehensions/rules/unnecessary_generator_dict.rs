use log::error;
use rustpython_parser::ast::{Expr, ExprKind, Keyword};

use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::types::Range;

use crate::checkers::ast::Checker;
use crate::registry::Diagnostic;
use crate::rules::flake8_comprehensions::fixes;
use crate::violation::AlwaysAutofixableViolation;

use super::helpers;

/// ## What it does
/// Checks for unnecessary generators that can be rewritten as `dict`
/// comprehensions.
///
/// ## Why is this bad?
/// It is unnecessary to use `dict` around a generator expression, since
/// there are equivalent comprehensions for these types. Using a
/// comprehension is clearer and more idiomatic.
///
/// ## Examples
/// ```python
/// dict((x, f(x)) for x in foo)
/// ```
///
/// Use instead:
/// ```python
/// {x: f(x) for x in foo}
/// ```
#[violation]
pub struct UnnecessaryGeneratorDict;

impl AlwaysAutofixableViolation for UnnecessaryGeneratorDict {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Unnecessary generator (rewrite as a `dict` comprehension)")
    }

    fn autofix_title(&self) -> String {
        "Rewrite as a `dict` comprehension".to_string()
    }
}

/// C402 (`dict((x, y) for x, y in iterable)`)
pub fn unnecessary_generator_dict(
    checker: &mut Checker,
    expr: &Expr,
    parent: Option<&Expr>,
    func: &Expr,
    args: &[Expr],
    keywords: &[Keyword],
) {
    let Some(argument) = helpers::exactly_one_argument_with_matching_function("dict", func, args, keywords) else {
        return;
    };
    if let ExprKind::GeneratorExp { elt, .. } = argument {
        match &elt.node {
            ExprKind::Tuple { elts, .. } if elts.len() == 2 => {
                let mut diagnostic =
                    Diagnostic::new(UnnecessaryGeneratorDict, Range::from_located(expr));
                if checker.patch(diagnostic.kind.rule()) {
                    match fixes::fix_unnecessary_generator_dict(
                        checker.locator,
                        checker.stylist,
                        expr,
                        parent,
                    ) {
                        Ok(fix) => {
                            diagnostic.amend(fix);
                        }
                        Err(e) => error!("Failed to generate fix: {e}"),
                    }
                }
                checker.diagnostics.push(diagnostic);
            }
            _ => {}
        }
    }
}
