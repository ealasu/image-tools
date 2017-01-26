#!/usr/bin/env python3

from astropy.stats import sigma_clipped_stats
from photutils import datasets
import matplotlib.pyplot as plt
from astropy.visualization import SqrtStretch
from astropy.visualization.mpl_normalize import ImageNormalize
from photutils import CircularAperture
from astropy.io.fits import ImageHDU
from photutils import DAOStarFinder

with open('a.fits', 'rb') as f:
  # >>> hdu = datasets.load_star_image()    
  hdu = ImageHDU.readfrom(f)
  # data = hdu.data
  data = hdu.data[1300:1800, 1500:2000]    

mean, median, std = sigma_clipped_stats(data, sigma=3.0, iters=5)    
print((mean, median, std))    


daofind = DAOStarFinder(fwhm=3.0, threshold=5.*std)    
sources = daofind(data - median)    
# print(sources)    



positions = (sources['xcentroid'], sources['ycentroid'])
apertures = CircularAperture(positions, r=8.)
norm = ImageNormalize(stretch=SqrtStretch())
plt.imshow(data, cmap='Greys', origin='lower', norm=norm)
apertures.plot(color='blue', lw=1.5, alpha=0.5)

plt.savefig('fig.jpg')
