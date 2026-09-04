Onoff plugin
===============

The `onoff` plugin is designed for pulsar analysis. It assigns phases to each frame, makes a stacked image for phases in the "on" range, and subtracts the image for phases in the "off" range. The on and off ranges are provided as the `--range` command line argument, in the form `START_ON:STOP_ON,START_OFF:STOP_OFF`. If START > STOP, the phase window is assumed to wrap around through 1. Note that a .par ephemeris file is necessary, which is used to assign phases using the `PINT` software.