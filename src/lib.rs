pub mod layout;
pub mod mathfont;
pub mod rendered_block;

pub use rendered_block::RenderedBlock;

use rust_latex_parser::parse_equation;

/// Parse a LaTeX math string and render it as a 2D character grid.
pub fn render(latex: &str) -> RenderedBlock {
    let ast = parse_equation(latex);
    layout::layout(&ast)
}
