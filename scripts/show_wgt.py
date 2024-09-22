#!/usr/bin/env python

import matplotlib.pylab as plt
import numpy as np
import sys


wgt=np.fromfile(sys.argv[1], dtype='<i2')
wgt=np.reshape((wgt[::2]+1j*wgt[1::2]).astype('complex64'), [512, 128])

plt.subplot(121)
plt.imshow(np.abs(wgt), aspect='auto')
plt.colorbar()
plt.title("linear amplitude")
plt.xlabel('Port No.')
plt.ylabel('Channel No.')
plt.subplot(122)
plt.imshow(np.degrees(np.angle(wgt)), aspect='auto')
plt.colorbar()
plt.title("phase ($^{\circ}$)")
plt.xlabel('Port No.')
plt.ylabel('Channel No.')
plt.tight_layout()
plt.show()

