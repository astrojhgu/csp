#!/usr/bin/env python3

import matplotlib

matplotlib.use('TkAgg')

import matplotlib.pylab as plt
import numpy as np
import glob
import os
import astropy.io.fits as pyfits
from datetime import datetime
import sys

def db(x):
    return np.log10(x)*10

def lin(x):
    return x



datestr=sys.argv[1]
base_ch=int(sys.argv[2])
n1=int(sys.argv[3])
n2=int(sys.argv[4])

df=240/512/16
freq=df*(base_ch*16+np.arange(1600))

#datestr=20240905

timestamps=np.array([datetime.fromtimestamp(int(i.strip().split()[0])/1000).strftime('%H:%M') for i in open(f'b_time_{datestr}.txt')])
raw_data1=np.fromfile(f"b_01_{datestr}.dat", dtype='complex64')
raw_data1=raw_data1.reshape((-1, 1600))



plt.figure(figsize=(20,30))
plt.imshow(db(np.abs(raw_data1[n1:n2,:])), aspect='auto',extent=(freq[0], freq[-1], n2,n1))
plt.yticks(np.arange(timestamps.shape[0])[n1:n2:1000], timestamps[n1:n2:1000])
#plt.xticks
#plt.imshow(np.abs(raw_data1[:,:]), aspect='auto')
#plt.colorbar()
plt.xlabel('freq (MHz)')
plt.ylabel('time')
plt.title(f'waterfall of {datestr}')
plt.tight_layout()
#plt.xlim(0,200)
plt.savefig(f'waterfall_{datestr}.jpg')


plt.figure(figsize=(20,30))
plt.imshow(lin(np.angle(raw_data1[n1:n2,:])), aspect='auto',extent=(freq[0], freq[-1], n2,n1))
plt.yticks(np.arange(timestamps.shape[0])[n1:n2:1000], timestamps[n1:n2:1000])
#plt.xticks
#plt.imshow(np.abs(raw_data1[:,:]), aspect='auto')
#plt.colorbar()
plt.xlabel('freq (MHz)')
plt.ylabel('time')
plt.title(f'phase of {datestr}')
plt.tight_layout()
#plt.xlim(0,200)
plt.savefig(f'phase_{datestr}.jpg')

