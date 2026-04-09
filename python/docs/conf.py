# Configuration file for the Sphinx documentation builder.
# https://www.sphinx-doc.org/en/master/usage/configuration.html

import os
import sys

# Make the term_maths package importable.
# The compiled extension (_term_maths.so) must be installed first:
#   pip install -e .   (from repo root, with maturin installed)
# or:
#   maturin develop --features python
sys.path.insert(0, os.path.abspath("../../python"))

# ---------------------------------------------------------------------------
# Project information
# ---------------------------------------------------------------------------

project = "term-maths"
author = "Jack Geraghty"
copyright = f"2024, {author}"
release = "0.1.0"

# ---------------------------------------------------------------------------
# General configuration
# ---------------------------------------------------------------------------

extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.napoleon",        # NumPy / Google-style docstrings
    "sphinx_autodoc_typehints",   # type hints from annotations / stubs
    "sphinx.ext.viewcode",
]

# sphinx-autodoc-typehints settings
always_document_param_types = True
typehints_fully_qualified = False
simplify_optional_unions = True

# autodoc settings
autoclass_content = "both"          # include both class and __init__ docstrings
autodoc_typehints = "description"   # render type hints in the description, not signature
autodoc_member_order = "bysource"

# ---------------------------------------------------------------------------
# HTML output
# ---------------------------------------------------------------------------

html_theme = "furo"
html_title = "term-maths"
html_theme_options = {
    "source_repository": "https://github.com/jmg049/term-maths",
    "source_branch": "main",
    "source_directory": "python/docs/",
}

# ---------------------------------------------------------------------------
# Source files
# ---------------------------------------------------------------------------

templates_path = ["_templates"]
exclude_patterns = ["_build"]
