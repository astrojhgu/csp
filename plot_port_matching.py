#!/usr/bin/env python
import matplotlib.pylab as plt
import numpy as np
import glob
import os
import sys


def db(x):
    return np.log10(x)*10


plt.figure(figsize=(12,6))

port=int(sys.argv[1])
for prefix in [f"port_matching/{i}_{port}" for i in ["on","off"]]:
    raw_data1=np.fromfile(f"{prefix}_00.dat", dtype='complex64')
    raw_data1=raw_data1.reshape((-1, 1600))
    raw_data2=np.fromfile(f"{prefix}_11.dat", dtype='complex64')
    raw_data2=raw_data2.reshape((-1, 1600))
    plt.plot(db(np.mean(np.abs(raw_data1[-10:,:])**2, axis=0)),'-', label=f"{prefix} beam 1")
    plt.plot(db(np.mean(np.abs(raw_data2[-10:,:])**2, axis=0)),'--', label=f"{prefix} beam 2")

plt.title(f"Port {port}")
plt.legend()
plt.ylim(10,120)
plt.savefig(f"port_matching/fig_{port}.png")
plt.show(block=False)
plt.pause(3)
plt.close()

