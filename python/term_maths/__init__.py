"""
term_maths — Character-grid mathematical notation renderer.

Renders LaTeX math expressions as 2D Unicode art suitable for terminal display.

Quick start::

    >>> import term_maths
    >>> print(term_maths.render(r"\\frac{a}{b}"))
     a
    ───
     b

The :class:`RenderedBlock` returned by :func:`render` can be composed further
using methods like :meth:`~RenderedBlock.beside`, :meth:`~RenderedBlock.pad`,
and :meth:`~RenderedBlock.center_in`.
"""

from ._term_maths import (
    RenderedBlock,
    render,
    to_latex,
    map_char,
    map_str,
)

__all__ = [
    "RenderedBlock",
    "render",
    "to_latex",
    "map_char",
    "map_str",
]
