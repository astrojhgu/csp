#!/usr/bin/env python3

import sys
import math
import cmath
import numpy as np
import astropy.io.fits as pyfits
from astropy.coordinates import (SkyCoord, Distance, Galactic, 
                                 EarthLocation, AltAz)
import astropy.coordinates as coord
from astropy.io import fits
from astropy.table import QTable
from astropy.time import Time
import astropy.units as u


import scipy.fftpack
from scipy.optimize import fmin
from datetime import datetime

c=2.99792458e8
nch=1600
prefix=sys.argv[1]
date_str=sys.argv[2]
base_ch=int(sys.argv[3])
uv_outname=f"uvr_{date_str}_{base_ch}.fits"
fmax=240e6
img_w=2048

lon=86.4225464546*u.deg
lat=42.5526813923*u.deg

ulastai_loc = EarthLocation.from_geodetic(
    lon=lon, lat=lat)

df=fmax/512/16
freq=df*(base_ch*16+np.arange(nch))



sidereal_angle=Time(np.array([datetime.fromtimestamp(int(i.strip().split()[0])/1000) for i in open(f'{prefix}_time_{date_str}.txt')]), location=ulastai_loc).sidereal_time('mean').value/24*np.pi*2

raw_data01=np.fromfile(f"{prefix}_01_{date_str}.dat", dtype='complex64')
raw_data00=np.fromfile(f"{prefix}_00_{date_str}.dat", dtype='complex64')
raw_data11=np.fromfile(f"{prefix}_11_{date_str}.dat", dtype='complex64')

raw_data01=raw_data01.reshape((-1, nch))
raw_data11=raw_data11.reshape((-1, nch))
raw_data00=raw_data00.reshape((-1, nch))

n=len(sidereal_angle)

raw_data01=raw_data01[:n,:]
raw_data00=raw_data00[:n,:]
raw_data11=raw_data11[:n,:]



corr_coeff=raw_data01/np.sqrt(np.real(raw_data00)*np.real(raw_data11))



def calc_uv(delay):
    result=np.zeros([img_w, img_w], dtype=np.complex64)
    wgt=np.zeros([img_w, img_w])+1e-9
    phases=np.exp(1j*delay/c*freq*2*np.pi)
    phases=np.tile(phases, [n, 1])
    corrected=corr_coeff*phases
    r=freq/freq[-1]*img_w/2
    for it in range(n):
        a=sidereal_angle[it]
        ca=np.cos(a)
        sa=np.sin(a)
        x=(r*ca).astype('int')+img_w//2
        y=(r*sa).astype('int')+img_w//2
        result[x,y]+=corrected[it, :]
        wgt[x,y]+=1
    result/=wgt
    #result+=np.conj(result[::-1,::-1])
    return result

def uv2img(uv):
    return np.fft.fftshift(np.fft.fft2(np.fft.fftshift(uv)))

delay=22
opt_delay=0
max_value=-1e99
uv=calc_uv(delay)
img=uv2img(uv)
m=np.max(img.real)
opt_delay=delay
max_value=m
pyfits.PrimaryHDU(img.real).writeto('m.fits',overwrite=True)
pyfits.PrimaryHDU(uv.real).writeto(uv_outname,overwrite=True)

#uv=calc_uv(1)
#img=uv2img(uv)
#pyfits.PrimaryHDU(img.real).writeto('a.fits',overwrite=True)

