term-maths Python API
=====================

**term-maths** renders LaTeX math expressions as 2D Unicode art for terminals.

.. code-block:: python

   import term_maths

   block = term_maths.render(r"\frac{a}{b}")
   print(block)
   #  a
   # ───
   #  b

   # Compose blocks
   lhs = term_maths.render(r"x^2")
   rhs = term_maths.render(r"y^2")
   sep = term_maths.RenderedBlock.from_text(" + ")
   combined = lhs.beside(sep).beside(rhs)
   print(combined)

   # Unicode math fonts
   print(term_maths.map_str("blackboard", "NZQRC"))  # ℕℤℚℝℂ

Contents
--------

.. toctree::
   :maxdepth: 2

   api
   examples

Installation
------------

Requires `maturin <https://www.maturin.rs>`_ and a Rust toolchain.

.. code-block:: sh

   pip install maturin
   maturin develop --features python   # from the repo root


Indices
-------

* :ref:`genindex`
* :ref:`modindex`
