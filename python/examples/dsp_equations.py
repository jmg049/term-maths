"""DSP equations demo — Python equivalent of examples/dsp_equations.rs."""

import term_maths

EQUATIONS = [
    (
        r"X[k] = \sum_{n=0}^{N-1} x[n] \cdot e^{-j \frac{2\pi}{N} kn}",
        "DFT Summation",
    ),
    (
        r"(f * g)(t) = \int_{-\infty}^{\infty} f(\tau) g(t - \tau) \, d\tau",
        "Convolution Integral",
    ),
    (
        r"H(z) = \frac{b_0 + b_1 z^{-1} + b_2 z^{-2}}{1 + a_1 z^{-1} + a_2 z^{-2}}",
        "Transfer Function",
    ),
    (
        r"w(n) = 0.5 \left(1 - \cos\left(\frac{2\pi n}{N - 1}\right)\right)",
        "Hann Window",
    ),
]

for latex, label in EQUATIONS:
    print(f"=== {label} ===")
    print(f"LaTeX: {latex}")
    print()
    print(term_maths.render(latex))
    print()

print("=== Standalone operators ===\n")

standalone = [
    (r"\sum_{n=0}^{N-1}", "Sum with limits"),
    (r"\int_{0}^{1}", "Integral with limits"),
    (r"\prod_{i=1}^{n}", "Product with limits"),
    (r"\left(\frac{a}{b}\right)", "Delimited fraction"),
    (r"\overline{x + y}", "Overline"),
    (r"\hat{x}", "Hat"),
    (r"\sqrt{\frac{a}{b}}", "Sqrt of fraction"),
]

for latex, label in standalone:
    print(f"--- {label} ---")
    print(term_maths.render(latex))
    print()
