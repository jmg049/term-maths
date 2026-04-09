use term_maths::render;

/// Helper: render and collect output lines, trimming trailing whitespace per line.
fn render_lines(latex: &str) -> Vec<String> {
    let block = render(latex);
    let output = format!("{}", block);
    output.lines().map(|l| l.trim_end().to_string()).collect()
}

#[test]
fn test_simple_fraction() {
    let lines = render_lines(r"\frac{a}{b}");
    assert_eq!(lines, vec![" a", "───", " b"]);
}

#[test]
fn test_nested_fraction() {
    let lines = render_lines(r"\frac{1}{1+\frac{1}{x}}");
    // Numerator "1" centered over denominator "1 + 1/x"
    assert_eq!(lines.len(), 5);
    // Top line: centered "1"
    assert!(lines[0].contains('1'));
    // Bar line
    assert!(lines[1].chars().all(|c| c == '─'));
    // Denominator contains nested fraction
    assert!(lines[3].contains('+'));
}

#[test]
fn test_superscript() {
    let lines = render_lines(r"x^2");
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].trim(), "2");
    assert_eq!(lines[1].trim(), "x");
}

#[test]
fn test_subscript() {
    let lines = render_lines(r"a_n");
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].trim(), "a");
    assert_eq!(lines[1].trim(), "n");
}

#[test]
fn test_supsub() {
    let lines = render_lines(r"x_i^2");
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0].trim(), "2");
    assert_eq!(lines[1].trim(), "x");
    assert_eq!(lines[2].trim(), "i");
}

#[test]
fn test_horizontal_sequence() {
    let block = render(r"a + b");
    // Should be a single row with spaces around operators
    assert_eq!(block.height(), 1);
    let output = format!("{}", block);
    assert!(output.contains("a"));
    assert!(output.contains("+"));
    assert!(output.contains("b"));
}

#[test]
fn test_sqrt() {
    let lines = render_lines(r"\sqrt{x}");
    // Should have overline and radical
    assert!(lines[0].contains('─'));
    assert!(lines.iter().any(|l| l.contains('√')));
    assert!(lines.iter().any(|l| l.contains('x')));
}

#[test]
fn test_euler_identity() {
    let lines = render_lines(r"e^{i\pi} + 1 = 0");
    // Should render as 2 rows (e with superscript iπ, then + 1 = 0)
    assert!(lines.len() >= 2);
    // Top row should have iπ
    let joined = lines.join("\n");
    assert!(joined.contains('π'));
    assert!(joined.contains('0'));
}

#[test]
fn test_quadratic_formula() {
    let lines = render_lines(r"\frac{-b \pm \sqrt{b^2 - 4ac}}{2a}");
    // Should be a multi-line fraction with sqrt in numerator
    assert!(lines.len() >= 3); // at least num + bar + den
    // Contains the fraction bar
    assert!(lines.iter().any(|l| l.contains('─') && !l.contains('√')));
    // Contains sqrt
    let joined = lines.join("\n");
    assert!(joined.contains('√'));
    assert!(joined.contains("2a"));
}

#[test]
fn test_empty_input() {
    let block = render("");
    assert!(block.is_empty() || block.height() <= 1);
}

#[test]
fn test_single_symbol() {
    let block = render(r"\alpha");
    assert_eq!(block.height(), 1);
    let output = format!("{}", block);
    assert!(output.contains('α'));
}

#[test]
fn test_fraction_baseline_alignment() {
    // When a fraction appears beside other content, baselines should align
    let lines = render_lines(r"x + \frac{a}{b}");
    // x and + should be on the fraction bar row
    let bar_row = lines.iter().position(|l| l.contains('─')).unwrap();
    assert!(lines[bar_row].contains('x') || lines[bar_row].contains('+'));
}
