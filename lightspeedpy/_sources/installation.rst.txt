Installing Lightspeedpy
=======================

Install Lightspeedpy with

.. code-block::
    
    git clone https://github.com/jack-dinsmore/lightspeedpy
    cd lightspeedpy
    python3 -m pip install -e .

The `-e` argument in the last line enables you to edit the `lightspeedpy` code and have the changes take effect.

If you intend to make your own plugins, we suggest first forking the lightspeedpy repository. That means copying the main lightspeedpy repository to a repository you own and can edit yourself. This is done using Github's website. See Github documentation for instructions. Then install your fork with

.. code-block::
    
    git clone YOUR_REPOSITORY_LINK
    cd lightspeedpy
    python3 -m pip install -e .