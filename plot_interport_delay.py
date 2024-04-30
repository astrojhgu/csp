#!/usr/bin/env python
import matplotlib.pylab as plt
import numpy as np
import glob
import os
import sys


def db(x):
    return np.log10(x)*10



port=int(sys.argv[1])

xx_off=np.fromfile(f"interport_delay/off_{port}_00.dat", dtype='complex64')
xx_off=xx_off.reshape((-1, 1600))
yy_off=np.fromfile(f"interport_delay/off_{port}_11.dat", dtype='complex64')
yy_off=yy_off.reshape((-1, 1600))
xy_off=np.fromfile(f"interport_delay/off_{port}_01.dat", dtype='complex64')
xy_off=xy_off.reshape((-1, 1600))

xx_on=np.fromfile(f"interport_delay/on_{port}_00.dat", dtype='complex64')
xx_on=xx_on.reshape((-1, 1600))
yy_on=np.fromfile(f"interport_delay/on_{port}_11.dat", dtype='complex64')
yy_on=yy_on.reshape((-1, 1600))
xy_on=np.fromfile(f"interport_delay/on_{port}_01.dat", dtype='complex64')
xy_on=xy_on.reshape((-1, 1600))


plt.figure(figsize=(12,12))
plt.subplot(411)
plt.title('phase off')
plt.plot(np.degrees(np.angle(np.mean(xy_off[-10:,:], axis=0))))
plt.ylabel('phase (deg)')
plt.ylim(-180,180)

plt.subplot(412)
plt.title('phase on')
plt.plot(np.degrees(np.angle(np.mean(xy_on[-10:,:], axis=0))))
plt.ylabel('phase (deg)')
plt.ylim(-30,30)

plt.subplot(413)
plt.title('ampl off')
plt.plot(db(np.mean(np.abs(xy_off[-10:,:])**2, axis=0)),label='xy')
plt.plot(db(np.mean(np.abs(xx_off[-10:,:])**2, axis=0)),label='xx')
plt.plot(db(np.mean(np.abs(yy_off[-10:,:])**2, axis=0)),label='yy')
plt.legend()

plt.subplot(414)
plt.title('ampl on')
plt.plot(db(np.mean(np.abs(xy_on[-10:,:])**2, axis=0)),label='xy')
plt.plot(db(np.mean(np.abs(xx_on[-10:,:])**2, axis=0)),label='xx')
plt.plot(db(np.mean(np.abs(yy_on[-10:,:])**2, axis=0)),label='yy')
plt.legend()

plt.savefig(f"interport_delay/fig_{port}.png")

plt.show()

#plt.show(block=False)
plt.tight_layout()
plt.pause(3)
plt.close()
