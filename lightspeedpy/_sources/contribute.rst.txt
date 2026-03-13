How to contribute to Lightspeedpy
=================================

Lightspeedpy consists of a core set of code and many plugins. The core code (stored in the main lightspeedpy directory) does common tasks like loading data sets and subtracting / dividing biases, darks and flats. Plugins (stored in subdirectories) perform more specialized tasks like stacking frames to form an image, or extracting a light curve.

Each plugin can be imported like a python library, and it can also be run like a command line tool for those that don't use python.

We're separating plugins and core code to avoid merge conflicts. The idea is each person edits their own plugin, and the core code should be stable enough that nobody needs to edit it. If you'd like to make your own plugin, do the following:

1. Copy the `template` directory and rename it to be the name of your plugin (don't use hyphens or periods in the name -- use underscores.)
2. Edit `__main__.py` to define the plugin's command line interface. See the file for additional instructions. The command is called by `python -m lightspeedpy.PLUGIN_NAME`.
3. Edit `__init__.py` to define the plugin's python API. See the file for additional instructions. The API is imported using `from lightspeedpy import PLUGIN_NAME`.
