PSF plugin
===============

The `psf` plugin fits a Gaussian PSF model to a specified region of a data cube. It outputs the PSF FWHM major and minor axes, and the orientation of its major axis measured counterclockwise from North.

This script is useful if you perform PSF weighting. To do so, construct an elliptical region centered on the source with the same ellipse parameters as those output by the `psf` plugin. (Note that DS9 requires you to input the ellipse semi-axes, and the `psf` script reports the full axes. So you will need to divide by 2). The PSF weighting script then fits to everything within 3 standard deviations of the ellipse center, and uses a Gaussian PSF model whose half-max contour is the region you provided.