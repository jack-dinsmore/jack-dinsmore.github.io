.. LeakageLib documentation master file, created by
   sphinx-quickstart on Fri Feb 13 14:55:39 2026.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

lightspeedpy documentation
==========================

Lightspeedpy is a data reduction library for the proto-Lightspeed instrument. It is capable of processing bias, darks, and flats, manipulating data files, and performing more complex tasks such as generating light curves.

Once installed, lightspeedpy can be imported as a Python library and its tools can be used programmatically. Alternatively, lightspeedpy can be used as a command line tool.

.. toctree::
   :maxdepth: 2
   :caption: Please review the below pages for how to install or contribute to lightspeedpy

   installation
   contribute
   

How do I use the command line interface (CLI)
---------------------------------------------
A CLI call to lightspeedpy takes the form

.. code-block::

   python -m lightspeedpy.TOOL_NAME ARGUMENTS

The list of available tools is accessible through lightspeedpy's help menu. To see it, run :code:`python -m lightspeedpy -h`.

To access the documentation of a specific tool, run :code:`python -m lightspeedpy.TOOL_NAME -h`. This will tell you what the tool does and how to provide arguments to the tool.

General notes
^^^^^^^^^^^^^

* **File names** If you pass in the name of a data cube `XXX_YYY_ZZZ_cubeNNN.fits`, `lightspeedpy` will automatically load in all other cubes from the same observation. That is, files with the same XXX, YYY, and ZZZ. If you only wish to load some of these cubes, you can use the max-index and min-index arguments. Then lightspeedpy will only load cubes with NNN between min-index and max-index, inclusive.
* **mode** `lightspeedpy` supports three modes of operation: `sum`, `clip`, and `weight`. Sub treats the QE-adjusted ADU received in each pixel as directly proportional to the number of electrons received. This is obviously wrong for any given pixel since the number of actual electrons must be an integer, but on average it is correct. `clip` rounds to the nearest whole number of electrons, which can reduce noise in some cases. `weight` attempts to perform a more complicated extraction which uses the distribution of read noise to infer the true number of electrons detected from the distribution of ADU received.
* **clobber** `lightspeedpy` will not overwrite output files by default. Set `clobber` if you do wish to overwrite these files
* **allow-cr** Use this argument to allow cosmic rays to enter your data (i.e., this argument turns off CR clipping). This will increase the analysis speed substantially.


How do I use the application programming interface (API)
--------------------------------------------------------
The lightspeedpy API can be accessed by importing lightspeedpy as a Python module. All the command line tools of lightspeedpy are available there as python functions. This API is particularly useful if you wish to test adding your own tools to lightspeedpy, or perform complex tasks.

The API is documented here:

.. toctree::
   :maxdepth: 2
   
   lightspeedpy