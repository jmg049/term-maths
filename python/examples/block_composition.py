"""
Block composition demo.

Shows how to build composite expressions by combining RenderedBlock objects
using beside(), pad(), center_in(), above(), and hline().
"""

import term_maths
from term_maths import RenderedBlock


def sep(text: str = " ") -> RenderedBlock:
    """Convenience: create a separator block from plain text."""
    return RenderedBlock.from_text(text)


# ---------------------------------------------------------------------------
# 1. Side-by-side composition aligned on baselines
# ---------------------------------------------------------------------------
print("=== Side-by-side (baseline-aligned) ===\n")

lhs = term_maths.render(r"\frac{a}{b}")
eq = sep(" = ")
rhs = term_maths.render(r"\frac{c}{d}")

print(lhs.beside(eq).beside(rhs))
print()

# A tall block beside a short one — short block sits on the baseline
tall = term_maths.render(r"\frac{1}{1 + \frac{1}{x}}")
plus = sep(" + ")
short = term_maths.render(r"y")

print(tall.beside(plus).beside(short))
print()

# ---------------------------------------------------------------------------
# 2. Horizontal centering under a fraction bar
# ---------------------------------------------------------------------------
print("=== Manual fraction construction ===\n")

numerator = term_maths.render(r"a + b")
denominator = term_maths.render(r"c + d")
bar_width = max(numerator.width, denominator.width) + 2
bar = RenderedBlock.hline("─", bar_width)

num_c = numerator.center_in(bar_width)
den_c = denominator.center_in(bar_width)

# Stack: numerator / bar / denominator; baseline is the bar row
fraction = RenderedBlock.above(num_c, bar, baseline_row=num_c.height)
fraction = RenderedBlock.above(fraction, den_c, baseline_row=num_c.height)

print(fraction)
print()

# ---------------------------------------------------------------------------
# 3. Padding and alignment
# ---------------------------------------------------------------------------
print("=== Padding ===\n")

block = term_maths.render(r"x^2 + y^2")
padded = block.pad(left=2, right=2, top=1, bottom=1)
print(f"Original ({block.width}×{block.height}):")
print(block)
print(f"\nPadded ({padded.width}×{padded.height}):")
print(padded)
print()

# ---------------------------------------------------------------------------
# 4. Accessing cells programmatically
# ---------------------------------------------------------------------------
print("=== Cell grid access ===\n")

block = term_maths.render(r"\frac{1}{2}")
print(f"RenderedBlock: width={block.width}, height={block.height}, baseline={block.baseline}")
print(f"repr: {block!r}")
print()

cells = block.cells()
for row_idx, row in enumerate(cells):
    marker = " <-- baseline" if row_idx == block.baseline else ""
    print(f"  row {row_idx}: {row}{marker}")
print()

# ---------------------------------------------------------------------------
# 5. Building a table of expressions
# ---------------------------------------------------------------------------
print("=== Expression table ===\n")

expressions = [
    r"\sum_{k=0}^{n} k",
    r"\frac{n(n+1)}{2}",
]

blocks = [term_maths.render(e) for e in expressions]
eq_sep = sep("  =  ")
composed = blocks[0].beside(eq_sep).beside(blocks[1])
print(composed)
print()
