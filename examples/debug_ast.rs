use rust_latex_parser::parse_equation;

fn main() {
    for expr in [r"a + b = c", r"e^{i\pi} + 1 = 0", r"-b \pm \sqrt{b^2 - 4ac}"] {
        println!("=== {} ===", expr);
        println!("{:#?}", parse_equation(expr));
        println!();
    }
}
