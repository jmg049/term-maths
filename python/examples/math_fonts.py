"""
Unicode mathematical font demo.

Shows how map_char() and map_str() transform ASCII letters and digits into
their Unicode Mathematical Alphanumeric Symbols equivalents.
"""

import term_maths

FONTS = ["bold", "blackboard", "calligraphic", "fraktur", "roman", "sans_serif", "monospace"]

ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"

print("=== Font map (uppercase A–Z) ===\n")
sample = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
for font in FONTS:
    mapped = term_maths.map_str(font, sample)
    print(f"  {font:<12}: {mapped}")

print()

print("=== Font map (lowercase a–z) ===\n")
sample = "abcdefghijklmnopqrstuvwxyz"
for font in FONTS:
    mapped = term_maths.map_str(font, sample)
    print(f"  {font:<12}: {mapped}")

print()

print("=== Font map (digits 0–9) ===\n")
sample = "0123456789"
for font in FONTS:
    mapped = term_maths.map_str(font, sample)
    print(f"  {font:<12}: {mapped}")

print()

print("=== Common mathematical sets ===\n")
sets = {
    "Naturals  ℕ": ("blackboard", "N"),
    "Integers  ℤ": ("blackboard", "Z"),
    "Rationals ℚ": ("blackboard", "Q"),
    "Reals     ℝ": ("blackboard", "R"),
    "Complex   ℂ": ("blackboard", "C"),
}
for label, (font, ch) in sets.items():
    print(f"  {label}  →  {term_maths.map_char(font, ch)}")

print()

print("=== Rendered with \\mathbb (via LaTeX parser) ===\n")
for expr, label in [
    (r"\mathbb{NZQRC}", "Common sets"),
    (r"\mathbf{v}", "Bold vector"),
    (r"\mathcal{L}", "Calligraphic L (Laplace)"),
    (r"\mathfrak{g}", "Fraktur g (Lie algebra)"),
]:
    print(f"  {label}: {term_maths.render(expr)}")

print()

print("=== Error handling ===\n")
try:
    term_maths.map_str("unknown_font", "hello")
except ValueError as e:
    print(f"  ValueError: {e}")
