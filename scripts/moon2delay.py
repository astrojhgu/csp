#!/usr/bin/env python3
import numpy as np
from astropy import units as u
from astropy.coordinates import (SkyCoord, Distance, Galactic,EarthLocation, AltAz)
import astropy.coordinates as coord
from astropy.io import fits
from astropy.table import QTable
from astropy.time import Time
from astropy.utils.data import download_file

from astropy.coordinates import get_moon

import sys
import yaml

import ant_pos

c=2.99792458e8

class Enable:
    def __init__(self, data):
        self.data=data


class Disable: 
    def __init__(self, data):
        self.data=data

class Default:
    def __init__(self):
        pass



def enable_representer(dumper, enable):
    return dumper.represent_sequence(u"!Enable", enable.data)

def enable_constructor(loader, node):
    value=loader.construct_sequence(node)
    return Enable(value)

def disable_representer(dumper, enable):
    return dumper.represent_sequence(u"!Disable", enable.data)

def disable_constructor(loader, node):
    value=loader.construct_sequence(node)
    return Enable(value)




yaml.add_representer(Enable, enable_representer)
yaml.add_constructor("!Enable", enable_constructor)

yaml.add_representer(Disable, disable_representer)
yaml.add_constructor("!Diable", disable_constructor)



'''
def altaz2vec(altaz):
    alt=np.radians((target.transform_to(altaz).alt/u.deg).value)
    az=np.radians((target.transform_to(altaz).az/u.deg).value)
    z=np.sin(alt)
    r=np.cos(alt)
    y=r*np.cos(az)
    x=r*np.sin(az)
    print(r)
    return np.array([x,y,z])
'''

def altaz2vec(altaz):
    alt=np.radians((altaz.alt/u.deg).value)
    az=np.radians((altaz.az/u.deg).value)
    z=np.sin(alt)
    r=np.cos(alt)
    y=r*np.cos(az)
    x=r*np.sin(az)
    print(r)
    return np.array([x,y,z])




lon=86.4225464546*u.deg
lat=42.5526813923*u.deg

ulastai_loc = EarthLocation.from_geodetic(
    lon=lon, lat=lat)

now=Time.now()
altaz = AltAz(location=ulastai_loc, obstime=now)
#target=SkyCoord(ra=ra, dec=dec, frame='icrs')
target=get_moon(now)
print(target.transform_to(altaz))


aa=target.transform_to(altaz)

alt=aa.alt.to(u.deg).value
if alt<40.0:
    sys.exit(1)
    
print(aa)
v=altaz2vec(aa)

if v[2]<0:
    sys.exit(1)

delay_m=ant_pos.data.dot(v)
delay_ns=delay_m/c/1e-9
yaml_data={
    "delay":delay_ns.tolist(),
    "ampl":[1]*128,
    "flags": Disable([])
}
yaml.dump(yaml_data, open("d.yaml",'w'))
