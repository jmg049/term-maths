use rust_latex_parser::{AccentKind, EqNode, MathFontKind, MatrixKind};

use crate::rendered_block::RenderedBlock;

/// Render an `EqNode` AST into a `RenderedBlock`.
pub fn layout(node: &EqNode) -> RenderedBlock {
    match node {
        EqNode::Text(s) => layout_text(s),
        EqNode::Space(_) => RenderedBlock::from_char(' '),
        EqNode::Seq(children) => layout_seq(children),
        EqNode::Frac(num, den) => layout_frac(num, den),
        EqNode::Sup(base, sup) => layout_sup(base, sup),
        EqNode::Sub(base, sub) => layout_sub(base, sub),
        EqNode::SupSub(base, sup, sub) => layout_supsub(base, sup, sub),
        EqNode::Sqrt(body) => layout_sqrt(body),
        EqNode::BigOp { symbol, lower, upper } => layout_bigop(symbol, lower, upper),
        EqNode::Accent(body, kind) => layout_accent(body, kind),
        EqNode::Limit { name, lower } => layout_limit(name, lower),
        EqNode::TextBlock(s) => RenderedBlock::from_text(s),
        EqNode::MathFont { kind, content } => layout_mathfont(kind, content),
        EqNode::Delimited { left, right, content } => layout_delimited(left, right, content),
        EqNode::Matrix { kind, rows } => layout_matrix(kind, rows),
        EqNode::Cases { rows } => layout_cases(rows),
        EqNode::Binom(top, bottom) => layout_binom(top, bottom),
        EqNode::Brace { content, label, over } => layout_brace(content, label, over),
        EqNode::StackRel { base, annotation, over } => layout_stackrel(base, annotation, over),
    }
}

fn layout_text(s: &str) -> RenderedBlock {
    RenderedBlock::from_text(s)
}

fn layout_seq(children: &[EqNode]) -> RenderedBlock {
    let mut result = RenderedBlock::empty();
    for child in children {
        let block = layout(child);
        result = result.beside(&block);
    }
    result
}

fn layout_frac(num: &EqNode, den: &EqNode) -> RenderedBlock {
    let num_block = layout(num);
    let den_block = layout(den);

    let bar_width = num_block.width().max(den_block.width()) + 2; // +2 for padding
    let bar = RenderedBlock::hline('─', bar_width);

    let num_centered = num_block.center_in(bar_width);
    let den_centered = den_block.center_in(bar_width);

    // Stack: numerator, bar, denominator. Baseline is the bar row.
    let top = RenderedBlock::above(&num_centered, &bar, 0);
    let baseline_row = top.height() - 1; // bar is the last row of 'top'
    RenderedBlock::above(&top, &den_centered, baseline_row)
}

fn layout_sup(base: &EqNode, sup: &EqNode) -> RenderedBlock {
    let base_block = layout(base);
    let sup_block = layout(sup);

    let rows = build_sup_sub_grid(
        base_block.cells(),
        base_block.width(),
        base_block.baseline(),
        sup_block.cells(),
        sup_block.width(),
        None,
        0,
    );

    let total_height = rows.len();
    let baseline = sup_block.height() + base_block.baseline() - 1;

    RenderedBlock::new(rows, baseline.min(total_height.saturating_sub(1)))
}

fn layout_sub(base: &EqNode, sub: &EqNode) -> RenderedBlock {
    let base_block = layout(base);
    let sub_block = layout(sub);

    let rows = build_sup_sub_grid(
        base_block.cells(),
        base_block.width(),
        base_block.baseline(),
        &[],
        0,
        Some((sub_block.cells(), sub_block.width())),
        0,
    );

    let total_height = rows.len();
    let baseline = base_block.baseline();

    RenderedBlock::new(rows, baseline.min(total_height.saturating_sub(1)))
}

fn layout_supsub(base: &EqNode, sup: &EqNode, sub: &EqNode) -> RenderedBlock {
    let base_block = layout(base);
    let sup_block = layout(sup);
    let sub_block = layout(sub);

    let rows = build_sup_sub_grid(
        base_block.cells(),
        base_block.width(),
        base_block.baseline(),
        sup_block.cells(),
        sup_block.width(),
        Some((sub_block.cells(), sub_block.width())),
        0,
    );

    let total_height = rows.len();
    let baseline = sup_block.height() + base_block.baseline() - 1;

    RenderedBlock::new(rows, baseline.min(total_height.saturating_sub(1)))
}

/// Build a grid for base with optional superscript above-right and subscript below-right.
///
/// Layout:
/// ```text
///          [sup rows]
///   [base] [overlap ]
///          [sub rows]
/// ```
///
/// The superscript's last row overlaps with the base's first row (right side).
/// The subscript's first row overlaps with the base's last row (right side).
fn build_sup_sub_grid(
    base_cells: &[Vec<String>],
    base_width: usize,
    _base_baseline: usize,
    sup_cells: &[Vec<String>],
    sup_width: usize,
    sub: Option<(&[Vec<String>], usize)>,
    _sub_baseline: usize,
) -> Vec<Vec<String>> {
    let base_height = base_cells.len();
    let sup_height = sup_cells.len();
    let (sub_cells, sub_width) = sub.unwrap_or((&[], 0));
    let sub_height = sub_cells.len();

    let script_width = sup_width.max(sub_width);

    // Sup rows above base (all but last sup row, which overlaps with base top)
    let sup_above = sup_height.saturating_sub(1);
    // Sub rows below base (all but first sub row, which overlaps with base bottom)
    let sub_below = sub_height.saturating_sub(1);

    let total_height = sup_above + base_height + sub_below;

    let mut rows = Vec::with_capacity(total_height);

    // Sup-only rows (above the base)
    for r in 0..sup_above {
        let mut row = vec![" ".to_string(); base_width];
        if r < sup_cells.len() {
            row.extend(sup_cells[r].iter().cloned());
            // Pad script column to script_width
            let used: usize = sup_cells[r].len();
            row.extend(std::iter::repeat_n(" ".to_string(), script_width.saturating_sub(used)));
        } else {
            row.extend(std::iter::repeat_n(" ".to_string(), script_width));
        }
        rows.push(row);
    }

    // Base rows
    for r in 0..base_height {
        let mut row = base_cells[r].clone();

        // Determine what goes in the script column for this row
        let sup_row_idx = sup_above + r; // index into sup if it extended this far
        let sub_row_idx_in_sub = r as isize - (base_height as isize - 1); // 0 for last base row

        if sup_row_idx < sup_height {
            // Last row(s) of sup overlapping with base
            row.extend(sup_cells[sup_row_idx].iter().cloned());
            let used = sup_cells[sup_row_idx].len();
            row.extend(std::iter::repeat_n(" ".to_string(), script_width.saturating_sub(used)));
        } else if sub_row_idx_in_sub >= 0 && (sub_row_idx_in_sub as usize) < sub_height {
            // First row(s) of sub overlapping with base
            let si = sub_row_idx_in_sub as usize;
            row.extend(sub_cells[si].iter().cloned());
            let used = sub_cells[si].len();
            row.extend(std::iter::repeat_n(" ".to_string(), script_width.saturating_sub(used)));
        } else {
            row.extend(std::iter::repeat_n(" ".to_string(), script_width));
        }

        rows.push(row);
    }

    // Sub-only rows (below the base)
    for r in 0..sub_below {
        let actual_sub_idx = sub_height - sub_below + r;
        let mut row = vec![" ".to_string(); base_width];
        if actual_sub_idx < sub_cells.len() {
            row.extend(sub_cells[actual_sub_idx].iter().cloned());
            let used = sub_cells[actual_sub_idx].len();
            row.extend(std::iter::repeat_n(" ".to_string(), script_width.saturating_sub(used)));
        } else {
            row.extend(std::iter::repeat_n(" ".to_string(), script_width));
        }
        rows.push(row);
    }

    rows
}

fn layout_sqrt(body: &EqNode) -> RenderedBlock {
    let body_block = layout(body);
    let body_h = body_block.height();

    // Build: overline on top, √ on left at bottom, │ extending up for tall bodies
    let overline_width = body_block.width();

    let mut rows = Vec::with_capacity(body_h + 1);

    // Top row: overline
    let mut top_row = vec![" ".to_string(); 1]; // space for radical column
    top_row.extend(std::iter::repeat_n("─".to_string(), overline_width));
    rows.push(top_row);

    // Body rows with radical on the left
    for r in 0..body_h {
        let radical_char = if r == body_h - 1 {
            "√".to_string()
        } else {
            "│".to_string()
        };
        let mut row = vec![radical_char];
        row.extend(body_block.cells()[r].iter().cloned());
        rows.push(row);
    }

    let baseline = 1 + body_block.baseline(); // shifted down by 1 for overline
    RenderedBlock::new(rows, baseline)
}

fn layout_bigop(
    symbol: &str,
    lower: &Option<Box<EqNode>>,
    upper: &Option<Box<EqNode>>,
) -> RenderedBlock {
    let op_block = RenderedBlock::from_text(symbol);

    let upper_block = upper.as_ref().map(|u| layout(u));
    let lower_block = lower.as_ref().map(|l| layout(l));

    let max_width = [
        op_block.width(),
        upper_block.as_ref().map_or(0, |b| b.width()),
        lower_block.as_ref().map_or(0, |b| b.width()),
    ]
    .into_iter()
    .max()
    .unwrap_or(1);

    let op_centered = op_block.center_in(max_width);

    let mut result = if let Some(ub) = upper_block {
        let ub_centered = ub.center_in(max_width);
        RenderedBlock::above(&ub_centered, &op_centered, ub_centered.height())
    } else {
        op_centered.clone()
    };

    let baseline = result.height() - 1; // operator row

    if let Some(lb) = lower_block {
        let lb_centered = lb.center_in(max_width);
        result = RenderedBlock::above(&result, &lb_centered, baseline);
    }

    // Baseline is at the operator symbol
    RenderedBlock::new(result.cells().to_vec(), baseline)
}

fn layout_accent(body: &EqNode, kind: &AccentKind) -> RenderedBlock {
    let body_block = layout(body);
    let accent_char = match kind {
        AccentKind::Hat => '^',
        AccentKind::Bar => '‾',
        AccentKind::Dot => '˙',
        AccentKind::DoubleDot => '¨',
        AccentKind::Tilde => '~',
        AccentKind::Vec => '→',
    };

    let accent_str = if body_block.width() <= 1 {
        RenderedBlock::from_char(accent_char)
    } else {
        // For bar/overline, repeat across width
        match kind {
            AccentKind::Bar => RenderedBlock::hline('‾', body_block.width()),
            _ => RenderedBlock::from_char(accent_char).center_in(body_block.width()),
        }
    };

    let baseline = accent_str.height() + body_block.baseline();
    RenderedBlock::above(&accent_str, &body_block, baseline)
}

fn layout_limit(name: &str, lower: &Option<Box<EqNode>>) -> RenderedBlock {
    let name_block = RenderedBlock::from_text(name);

    if let Some(low) = lower {
        let low_block = layout(low);
        let max_width = name_block.width().max(low_block.width());
        let name_centered = name_block.center_in(max_width);
        let low_centered = low_block.center_in(max_width);
        let baseline = name_centered.height() - 1;
        RenderedBlock::above(&name_centered, &low_centered, baseline)
    } else {
        name_block
    }
}

fn layout_mathfont(_kind: &MathFontKind, content: &EqNode) -> RenderedBlock {
    // Sprint 1: render content normally, font styling deferred to Sprint 3
    layout(content)
}

fn layout_delimited(left: &str, right: &str, content: &EqNode) -> RenderedBlock {
    let content_block = layout(content);
    let h = content_block.height();

    let left_block = build_delimiter(left, h);
    let right_block = build_delimiter(right, h);

    left_block.beside(&content_block).beside(&right_block)
}

/// Build a vertically-scaled delimiter.
fn build_delimiter(delim: &str, height: usize) -> RenderedBlock {
    if delim == "." || delim.is_empty() {
        // Invisible delimiter
        return RenderedBlock::new(
            vec![vec![" ".to_string()]; height],
            height / 2,
        );
    }

    if height <= 1 {
        return RenderedBlock::new(
            vec![vec![delim.to_string()]],
            0,
        );
    }

    let (top, mid, bot) = match delim {
        "(" => ("⎛", "⎜", "⎝"),
        ")" => ("⎞", "⎟", "⎠"),
        "[" => ("⎡", "⎢", "⎣"),
        "]" => ("⎤", "⎥", "⎦"),
        "{" => ("⎧", "⎨", "⎩"),
        "}" => ("⎫", "⎬", "⎭"),
        "|" => ("│", "│", "│"),
        "‖" => ("‖", "‖", "‖"),
        _ => (delim, delim, delim),
    };

    let mut rows = Vec::with_capacity(height);
    rows.push(vec![top.to_string()]);
    for _ in 1..height.saturating_sub(1) {
        rows.push(vec![mid.to_string()]);
    }
    if height > 1 {
        rows.push(vec![bot.to_string()]);
    }

    RenderedBlock::new(rows, height / 2)
}

fn layout_matrix(kind: &MatrixKind, matrix_rows: &[Vec<EqNode>]) -> RenderedBlock {
    if matrix_rows.is_empty() {
        return RenderedBlock::empty();
    }

    // Render all cells
    let rendered: Vec<Vec<RenderedBlock>> = matrix_rows
        .iter()
        .map(|row| row.iter().map(|cell| layout(cell)).collect())
        .collect();

    let num_cols = rendered.iter().map(|r| r.len()).max().unwrap_or(0);

    // Compute column widths and row heights
    let mut col_widths = vec![0usize; num_cols];
    let mut row_heights = vec![0usize; rendered.len()];

    for (r, row) in rendered.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            col_widths[c] = col_widths[c].max(cell.width());
            row_heights[r] = row_heights[r].max(cell.height());
        }
    }

    // Build grid row by row
    let col_sep = 1; // space between columns
    let mut grid = RenderedBlock::empty();

    for row in &rendered {
        let mut row_block = RenderedBlock::empty();
        for (c, cell) in row.iter().enumerate() {
            let padded = cell.center_in(col_widths[c]);
            if !row_block.is_empty() {
                row_block = row_block.beside(&RenderedBlock::from_text(&" ".repeat(col_sep)));
            }
            row_block = row_block.beside(&padded);
        }
        if grid.is_empty() {
            grid = row_block;
        } else {
            grid = RenderedBlock::above(&grid, &row_block, grid.height() / 2);
        }
    }

    // Set baseline to middle
    let total_height = grid.height();
    let grid = RenderedBlock::new(grid.cells().to_vec(), total_height / 2);

    // Wrap with delimiters based on matrix kind
    let (left, right) = match kind {
        MatrixKind::Paren => ("(", ")"),
        MatrixKind::Bracket => ("[", "]"),
        MatrixKind::Brace => ("{", "}"),
        MatrixKind::VBar => ("|", "|"),
        MatrixKind::DoubleVBar => ("‖", "‖"),
        MatrixKind::Plain => ("", ""),
    };

    if left.is_empty() {
        grid
    } else {
        let left_d = build_delimiter(left, total_height);
        let right_d = build_delimiter(right, total_height);
        left_d.beside(&grid).beside(&right_d)
    }
}

fn layout_cases(rows: &[(EqNode, Option<EqNode>)]) -> RenderedBlock {
    // Render as a left-brace delimited set of rows
    let rendered: Vec<RenderedBlock> = rows
        .iter()
        .map(|(val, cond)| {
            let val_block = layout(val);
            if let Some(c) = cond {
                let cond_block = layout(c);
                val_block
                    .beside(&RenderedBlock::from_text("  if "))
                    .beside(&cond_block)
            } else {
                val_block
            }
        })
        .collect();

    let max_width = rendered.iter().map(|b| b.width()).max().unwrap_or(0);
    let mut grid = RenderedBlock::empty();
    for row_block in &rendered {
        let padded = RenderedBlock::new(row_block.cells().to_vec(), row_block.baseline());
        // Pad to max width
        let full_row = RenderedBlock::new(
            padded.cells().iter().map(|r| {
                let mut r = r.clone();
                r.extend(std::iter::repeat_n(
                    " ".to_string(),
                    max_width.saturating_sub(r.len()),
                ));
                r
            }).collect(),
            padded.baseline(),
        );
        if grid.is_empty() {
            grid = full_row;
        } else {
            grid = RenderedBlock::above(&grid, &full_row, grid.height() / 2);
        }
    }

    let total_height = grid.height();
    let grid = RenderedBlock::new(grid.cells().to_vec(), total_height / 2);
    let left_brace = build_delimiter("{", total_height);
    left_brace.beside(&grid)
}

fn layout_binom(top: &EqNode, bottom: &EqNode) -> RenderedBlock {
    // Render as a fraction with parentheses instead of a bar
    let top_block = layout(top);
    let bot_block = layout(bottom);

    let inner_width = top_block.width().max(bot_block.width());
    let top_centered = top_block.center_in(inner_width);
    let bot_centered = bot_block.center_in(inner_width);

    let baseline = top_centered.height();
    let stacked = RenderedBlock::above(&top_centered, &bot_centered, baseline - 1);

    let h = stacked.height();
    let left = build_delimiter("(", h);
    let right = build_delimiter(")", h);
    left.beside(&stacked).beside(&right)
}

fn layout_brace(content: &EqNode, label: &Option<Box<EqNode>>, over: &bool) -> RenderedBlock {
    let content_block = layout(content);
    let w = content_block.width();

    // Build a horizontal brace
    let brace_str = if *over { "⏞" } else { "⏟" };
    let brace_block = RenderedBlock::hline(brace_str.chars().next().unwrap(), w);

    if let Some(lbl) = label {
        let label_block = layout(lbl).center_in(w);
        if *over {
            let top = RenderedBlock::above(&label_block, &brace_block, label_block.height());
            let baseline = top.height() + content_block.baseline();
            RenderedBlock::above(&top, &content_block, baseline)
        } else {
            let bottom = RenderedBlock::above(&brace_block, &label_block, 0);
            let baseline = content_block.baseline();
            RenderedBlock::above(&content_block, &bottom, baseline)
        }
    } else {
        if *over {
            let baseline = brace_block.height() + content_block.baseline();
            RenderedBlock::above(&brace_block, &content_block, baseline)
        } else {
            let baseline = content_block.baseline();
            RenderedBlock::above(&content_block, &brace_block, baseline)
        }
    }
}

fn layout_stackrel(base: &EqNode, annotation: &EqNode, over: &bool) -> RenderedBlock {
    let base_block = layout(base);
    let ann_block = layout(annotation);
    let w = base_block.width().max(ann_block.width());
    let base_centered = base_block.center_in(w);
    let ann_centered = ann_block.center_in(w);

    if *over {
        let baseline = ann_centered.height() + base_block.baseline();
        RenderedBlock::above(&ann_centered, &base_centered, baseline)
    } else {
        let baseline = base_block.baseline();
        RenderedBlock::above(&base_centered, &ann_centered, baseline)
    }
}
