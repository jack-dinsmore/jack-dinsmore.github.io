How to contribute to Lightspeedpy
=================================

In order to make your own plugin, perform the following steps.

0. (*optional*): Make a fork of the lightspeedpy git repository and install it (see :doc:`installation`). This step is recommended only if you'd eventually like to request that your plugin become an official part of lightspeedpy.
1. Copy the `template` directory and rename it to be the name of your plugin (don't use hyphens or periods.)
2. (*Make the command line utility*) Edit `__main__.py` to define the plugin's command line interface. See the file for additional instructions.
3. (*Make the programming interface*) Edit `__init__.py` to define the plugin's python API. See the file for additional instructions. The API is imported using `from lightspeedpy import PLUGIN_NAME`.
4. (*Add documentation, optional*) Add any example code or documentation to the `docs` directory.

If you have installed lightspeedpy with the `-e` argument as this documentation suggests, then you should now be able to use your plugin as though it were any other lightspeedpy plugin.

Specifically, the command line instructions  `python -m lightspeedpy.PLUGIN_NAME` will run the code you wrote in step 2, and `from lightspeedpy import PLUGIN_NAME` will import the API you defined in step 3. If you complete step 4 and run `make html` from lightspeedpy's main `docs` directory, then your packages documentation will also be made locally. The HTML page is stored at `docs/_build/index.html`.

Making your plugin publicly available
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

This is where the fork you created in step 0 becomes important. Use `git` tools to commit the local changes to your lightspeedpy fork. Then use Github to create a pull request. Github has good documentation about how to do this.

Pull requests are requests that your code be added (pulled) to the main lightspeedpy repository. The authors can accept or reject, and make comments.