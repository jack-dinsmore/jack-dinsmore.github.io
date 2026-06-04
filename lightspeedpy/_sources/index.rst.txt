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


How do I use the application programming interface (API)
--------------------------------------------------------
The lightspeedpy API can be accessed by importing lightspeedpy as a Python module. All the command line tools of lightspeedpy are available there as python functions. This API is particularly useful if you wish to test adding your own tools to lightspeedpy, or perform complex tasks.

The API is documented here:

.. toctree::
   :maxdepth: 2
   
   lightspeedpy