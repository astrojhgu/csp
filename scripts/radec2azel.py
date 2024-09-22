#!/usr/bin/env python3
import numpy as np
from astropy import units as u
from astropy.coordinates import (SkyCoord, Distance, Galactic, 
                                 EarthLocation, AltAz)
import astropy.coordinates as coord
from astropy.io import fits
from astropy.table import QTable
from astropy.time import Time
from astropy.utils.data import download_file

import sys


ra=float(sys.argv[1])*u.deg
dec=float(sys.argv[2])*u.deg

lon=86.4225464546*u.deg
lat=42.5526813923*u.deg

ulastai_loc = EarthLocation.from_geodetic(
    lon=lon, lat=lat)

now=Time.now()
altaz = AltAz(location=ulastai_loc, obstime=now)
target=SkyCoord(ra=ra, dec=dec, frame='icrs')

def altaz2vec(altaz):
    alt=np.radians((altaz.alt/u.deg).value)
    az=np.radians((altaz.az/u.deg).value)
    z=np.sin(alt)
    r=np.cos(alt)
    y=r*np.cos(az)
    x=r*np.sin(az)
    print(r)
    return np.array([x,y,z])


aa=target.transform_to(altaz)
print(aa)
v=altaz2vec(aa)
print(v)
print(np.dot(v,v))

