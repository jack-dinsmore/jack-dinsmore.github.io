.. LeakageLib documentation master file, created by
   sphinx-quickstart on Fri Feb 13 14:55:39 2026.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

lightspeedpy documentation
==========================

Lightspeedpy is a data reduction library for the proto-Lightspeed instrument. It is capable of processing bias, darks, and flats, manipulating data files, and performing more complex tasks such as generating light curves.

Lightspeedpy operates on a plugin system. The core code (stored in the main lightspeedpy directory) does common tasks like loading data sets and handling biases, darks and flats. Plugins (stored in subdirectories) perform more specialized tasks like stacking frames to form an image, or extracting a light curve. This design was chosen to make it easy to update and add functionality to lightspeedpy as its user base expands.

Once installed, lightspeedpy can be imported as a Python library and its tools can be used programmatically (each plugin will appear as a subpackage of lightspeedpy). Alternatively, lightspeedpy can be used as a command line tool(each plugin appears as a subcommand). For more details,including installation instructions, examples, and instructions on how to contribute to lightspeedpy, please see below.

.. toctree::
   :maxdepth: 2
   :caption: Contents

   installation
   cli
   api
   contribute

For instructions on each of lightspeedpy's plugins, please see

.. toctree::
   :maxdepth: 2
   :caption: Plugin examples
   :glob:

   corecode/*/docs/*
