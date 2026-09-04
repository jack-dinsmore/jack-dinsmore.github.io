LC plugin
===============

The `lc` plugin creates a light curve for periodic sources proto-Lightspeed data. An ephemeris `.par` file is necessary to assign phases to each image using the `PINT` software package. 

At present, each image is treated as instantaneous, which means you should not bin the light curve with bins smaller than the exposure times.

You can also get bootstrapped errors for your light curve by passing the :code:`--errors` flag. By default, 16 bootstrap samples will be run.

Adding light curves
^^^^^^^^^^^^^^^^^^^

You can add two light curves generated from multiple sources. The error bars will be added in quadrature. To do this, run

.. code-block::
    
    python -m lightspeedpy.lc add --inputs INPUTS --output OUTPUT
        
where `--inputs` is a space-separated list of input light curve files.