//! Demonstrates the crossterm renderer backend.
//!
//! Run with: cargo run --example crossterm_demo --features crossterm

#[cfg(feature = "crossterm")]
fn main() -> std::io::Result<()> {
    use std::io::{stdout, Write};
    use crossterm::{cursor, execute, terminal};
    use term_maths::{render, CrosstermRenderer};

    let block = render(r"\frac{-b \pm \sqrt{b^2 - 4ac}}{2a}");

    let mut stdout = stdout();

    // Print a header
    println!("Crossterm renderer demo — quadratic formula:\n");

    // Get current cursor position (approximate: after the println)
    let (col, row) = cursor::position()?;
    CrosstermRenderer::render_at(&mut stdout, &block, col, row)?;

    // Move cursor below the rendered block
    execute!(stdout, cursor::MoveTo(0, row + block.height() as u16 + 1))?;
    println!();

    Ok(())
}

#[cfg(not(feature = "crossterm"))]
fn main() {
    eprintln!("This example requires the `crossterm` feature.");
    eprintln!("Run with: cargo run --example crossterm_demo --features crossterm");
}
