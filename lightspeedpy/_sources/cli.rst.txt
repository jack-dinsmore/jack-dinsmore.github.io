
How do I use the command line interface (CLI)
---------------------------------------------
A CLI call to lightspeedpy takes the form

.. code-block::

   python -m lightspeedpy.TOOL_NAME ARGUMENTS

The list of available tools is accessible through lightspeedpy's help menu. To see it, run :code:`python -m lightspeedpy -h`.

To access the documentation of a specific tool, run :code:`python -m lightspeedpy.TOOL_NAME -h`. This will tell you what the tool does and how to provide arguments to the tool.

General notes
^^^^^^^^^^^^^

There are some command line flags which nearly all lightspeedpy plugins use. They are described in detail below.

* :code:`--input``: Pass the name of a data cube `XXX_YYY_ZZZ_cubeNNN.fits`. By default, lightspeedpy will automatically load in all other cubes from the same observation. That is, files with the same XXX, YYY, and ZZZ. If you only wish to load some of these cubes, you can use the :code:`--max-index` and :code:`--min-index` command line arguments. Then lightspeedpy will only load cubes with NNN between min-index and max-index, inclusive.

.. warning::
   The above :code:`--input` convention is potentially confusing. Every lightspeedpy plugin prints out the names of the files being read. Please check to make sure they are the files you expected. Ellipses denote consecutive cubes.

* :code:`--mode` lightspeedpy supports three modes of operation --- `sum`, `clip`, and `weight` --- which basically establish the way lightspeedpy infers the true source flux from the raw pixel output. `Sum` just adds the detected pixel output to estimate the source flux. `clip` first rounds to the nearest whole number of electrons, which can reduce noise in some cases. `weight` fits the source flux to the distribution of detected counts, and this is the most accurate method.
* :code:`--clobber` lightspeedpy will not overwrite output files by default. Set `clobber` if you do wish to overwrite these files
* :code:`--allow-cr` Use this argument to allow cosmic rays to enter your data (i.e., this argument turns off CR clipping). This will increase the analysis speed substantially.

Other notes
^^^^^^^^^^^

* **regions**: Lightspeedpy only supports single regions at the moment. You cannot add and subtract them. The regions should be in physical coordinates and ciao-formatted. The supported shapes are: circle, ellipse, box, polygon, annulus, and elliptical annulus.

* **progress bar colors**: Bias stacking has green and yellow progress bars in lightspeedpy. (Normal bias stacking is green, noise fitting is yellow.) The other plugins' operations are white. If you are running multiple analyses that use the same bias data, and the green and yellow bars take a long time to complete, consider using the `bias` plugin to make a processed bias file, and load that file instead.