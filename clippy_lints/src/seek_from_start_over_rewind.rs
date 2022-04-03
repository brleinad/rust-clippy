use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.61.0"]
    pub SEEK_FROM_START_OVER_REWIND,
    pedantic,
    "default lint description"
}
declare_lint_pass!(SeekFromStartOverRewind => [SEEK_FROM_START_OVER_REWIND]);

impl LateLintPass<'_> for SeekFromStartOverRewind {}