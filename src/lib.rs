pub mod latex_renderer;
pub mod layout;
pub mod mathfont;
pub mod rendered_block;
pub mod renderer;

#[cfg(feature = "crossterm")]
pub mod crossterm_renderer;

#[cfg(feature = "ratatui")]
pub mod ratatui_widget;

pub use rendered_block::RenderedBlock;
pub use renderer::{MathRenderer, TerminalRenderer};
pub use latex_renderer::LatexRenderer;

#[cfg(feature = "crossterm")]
pub use crossterm_renderer::CrosstermRenderer;

#[cfg(feature = "ratatui")]
pub use ratatui_widget::MathWidget;

use rust_latex_parser::parse_equation;

/// Parse a LaTeX math string and render it as a 2D character grid.
pub fn render(latex: &str) -> RenderedBlock {
    let ast = parse_equation(latex);
    layout::layout(&ast)
}

/// Parse a LaTeX math string and serialise it back to LaTeX (round-trip).
pub fn to_latex(latex: &str) -> String {
    let ast = parse_equation(latex);
    let renderer = LatexRenderer;
    renderer.render(&ast)
}
