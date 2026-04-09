Examples
========

All examples are in ``python/examples/`` and can be run after installing the package:

.. code-block:: sh

   maturin develop --features python
   python python/examples/render_demo.py


Render demo
-----------

Basic rendering of common mathematical expressions.

.. literalinclude:: ../../examples/render_demo.py
   :language: python
   :caption: python/examples/render_demo.py

DSP equations
-------------

Signal-processing formulae including the DFT, convolution integral, IIR transfer
function, and Hann window.

.. literalinclude:: ../../examples/dsp_equations.py
   :language: python
   :caption: python/examples/dsp_equations.py

Block composition
-----------------

Demonstrates how to combine :class:`~term_maths.RenderedBlock` objects using
:meth:`~term_maths.RenderedBlock.beside`, :meth:`~term_maths.RenderedBlock.pad`,
:meth:`~term_maths.RenderedBlock.center_in`, :meth:`~term_maths.RenderedBlock.above`,
and :meth:`~term_maths.RenderedBlock.hline`.

.. literalinclude:: ../../examples/block_composition.py
   :language: python
   :caption: python/examples/block_composition.py

Unicode math fonts
------------------

Shows :func:`~term_maths.map_char` and :func:`~term_maths.map_str` in action across
all supported font styles: bold, blackboard (double-struck), calligraphic, fraktur,
roman, sans-serif, and monospace.

.. literalinclude:: ../../examples/math_fonts.py
   :language: python
   :caption: python/examples/math_fonts.py
