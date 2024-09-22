#!/usr/bin/env python
import numpy as np
from scipy.optimize import fminbound
import matplotlib
matplotlib.use('svg')
import matplotlib.pylab as plt
import sys

infile=sys.argv[1]
ref_port=int(sys.argv[2]) if len(sys.argv)>2 else 0

ch_min=100
ch_max=400


nch=512
nport=128
max_delay=512

freq=np.arange(nch)/nch*0.5

def calc_phase(delay):
    return np.exp(1j*freq*2*np.pi*delay)

def fobj(delay, *args):
    data=args[0]
    ax=None
    if len(args)>1:
        ax=args[1]
    a=np.abs(data)
    #m=a>0
    d=data[ch_min:ch_max]/a[ch_min:ch_max]
    result=np.sum(np.abs(calc_phase(delay)[ch_min:ch_max]-d)**2)
    if ax:
        f=np.arange(nch)[ch_min:ch_max]
        print(f"result: {delay} {result}")
        ax.plot(f, np.degrees(np.angle(calc_phase(delay)[ch_min:ch_max])))
        ax.plot(f, np.degrees(np.angle(d)))
        ax.set_xlim((0, nch))
        ax.set_ylim((-180,180))
    return result


def regulate(data, ref_ch):
    ref_data=np.repeat(data[ref_ch, :], data.shape[0]).reshape((-1, data.shape[0])).T
    print(ref_data.shape)
    m=np.abs(data)==0
    data/=ref_data
    data[m]=0


    



data=np.fromfile(infile, dtype='<i2')
data=data[::2]+1j*data[1::2]

data=np.sum(data.reshape((-1,4)), axis=1)

data=data.reshape((nch,nport)).T
#data[:,:100]=0
#data[:, 400:]=0
print(data[101,101])
regulate(data, 0)
plt.figure(figsize=(20,15))
plt.imshow(np.abs(data), aspect='auto')

for port in range(nport):
    plt.text(x=100, y=port+0.5, s=f"{port}", fontsize=8)

plt.colorbar()
plt.tight_layout()
plt.savefig("ampl.pdf")
plt.close()


fig,ax=plt.subplots(figsize=(25,20))
#plt.imshow(np.angle(data), aspect='auto')

#x=fminbound(fobj, -100,100, args=(data[1,:],False))

im=ax.imshow(np.degrees(np.angle(data)), aspect='auto')

delay=[0]*nport
for port in range(nport):
    r=1e99
    x=0
    for d in np.arange(-max_delay, max_delay):
        r1=fobj(d, data[port,:])
        if r1<r:
            r=r1
            x=d
            #print(x, r)
    d1=fminbound(fobj, x-1, x+1, args=(data[port, :], ))
    delay[port]=d1
    print(f"{port}: {d1} {fobj(d1, data[port,:], )}")
    ax.text(x=405, y=port+0.5, s=f"{d1:.2f}", fontsize=8)
    ax.text(x=100, y=port+0.5, s=f"{port}", fontsize=8)
plt.colorbar(im)
#ax.tight_layout()
fig.savefig("phase.pdf")
#plt.show()
plt.close(fig)



nrows=16
ncols=8

fig,ax=plt.subplots(figsize=(25,20),nrows=nrows, ncols=ncols)
for i in range(nrows):
    for j in range(ncols):
        p=i*ncols+j
        fobj(delay[p], data[p,:], ax[i,j])
        ax[i,j].text(x=50, y=0, s=f"{p}-{i}-{j}")

fig.savefig("phase1.pdf")
plt.close(fig)
