#!/usr/bin/env python3

from astropy.stats import sigma_clipped_stats
from photutils import datasets
import matplotlib.pyplot as plt
from astropy.visualization import SqrtStretch
from astropy.visualization.mpl_normalize import ImageNormalize
from photutils import CircularAperture
from astropy.io.fits import ImageHDU
from photutils import DAOStarFinder

with open('crop.fits', 'rb') as f:
  hdu = ImageHDU.readfrom(f)
  data = hdu.data

# h = 3670
# h = 500
h = 2000

positions = ([], [])
with open('stars.txt', 'r') as f:
  for line in f:
    s = line.split(',')
    positions[0].append(float(s[0]))
    positions[1].append(float(h) - float(s[1]))
  
apertures = CircularAperture(positions, r=8.)
norm = ImageNormalize(stretch=SqrtStretch())
plt.imshow(data, cmap='Greys', origin='lower', norm=norm)
apertures.plot(color='blue', lw=1.5, alpha=0.5)

plt.savefig('fig.jpg')
