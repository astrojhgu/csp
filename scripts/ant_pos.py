import numpy as np

data=np.array([[-9.093267, -7.708793, 0],
 [-9.093267, -5.139196, 0],
 [-9.093267, -2.569598, 0],
 [-9.093267, 0.0, 0],
 [-9.093267, 2.569598, 0],
 [-9.093267, 5.139196, 0],
 [-9.093267, 7.708793, 0],
 [-7.577722, -8.993592, 0],
 [-7.577722, -6.423994, 0],
 [-7.577722, -3.854397, 0],
 [-7.577722, -1.284799, 0],
 [-7.577722, 1.284799, 0],
 [-7.577722, 3.854397, 0],
 [-7.577722, 6.423994, 0],
 [-7.577722, 8.993592, 0],
 [-6.062178, -10.278391, 0],
 [-6.062178, -7.708793, 0],
 [-6.062178, -5.139196, 0],
 [-6.062178, -2.569598, 0],
 [-6.062178, 0.0, 0],
 [-6.062178, 2.569598, 0],
 [-6.062178, 5.139196, 0],
 [-6.062178, 7.708793, 0],
 [-6.062178, 10.278391, 0],
 [-4.546633, -11.56319, 0],
 [-4.546633, -8.993592, 0],
 [-4.546633, -6.423994, 0],
 [-4.546633, -3.854397, 0],
 [-4.546633, -1.284799, 0],
 [-4.546633, 1.284799, 0],
 [-4.546633, 3.854397, 0],
 [-4.546633, 6.423994, 0],
 [-4.546633, 8.993592, 0],
 [-4.546633, 11.56319, 0],
 [-3.031089, -12.847989, 0],
 [-3.031089, -10.278391, 0],
 [-3.031089, -7.708793, 0],
 [-3.031089, -5.139196, 0],
 [-3.031089, -2.569598, 0],
 [-3.031089, 0.0, 0],
 [-3.031089, 2.569598, 0],
 [-3.031089, 5.139196, 0],
 [-3.031089, 7.708793, 0],
 [-3.031089, 10.278391, 0],
 [-3.031089, 12.847989, 0],
 [-1.515544, -14.132788, 0],
 [-1.515544, -11.56319, 0],
 [-1.515544, -8.993592, 0],
 [-1.515544, -6.423994, 0],
 [-1.515544, -3.854397, 0],
 [-1.515544, -1.284799, 0],
 [-1.515544, 1.284799, 0],
 [-1.515544, 3.854397, 0],
 [-1.515544, 6.423994, 0],
 [-1.515544, 8.993592, 0],
 [-1.515544, 11.56319, 0],
 [-1.515544, 14.132788, 0],
 [0.0, -15.417587, 0],
 [0.0, -12.847989, 0],
 [0.0, -10.278391, 0],
 [0.0, -7.708793, 0],
 [0.0, -5.139196, 0],
 [0.0, -2.569598, 0],
 [0.0, 0.0, 0],
 [0.0, 2.569598, 0],
 [0.0, 5.139196, 0],
 [0.0, 7.708793, 0],
 [0.0, 10.278391, 0],
 [0.0, 12.847989, 0],
 [0.0, 15.417587, 0],
 [1.515544, -14.132788, 0],
 [1.515544, -11.56319, 0],
 [1.515544, -8.993592, 0],
 [1.515544, -6.423994, 0],
 [1.515544, -3.854397, 0],
 [1.515544, -1.284799, 0],
 [1.515544, 1.284799, 0],
 [1.515544, 3.854397, 0],
 [1.515544, 6.423994, 0],
 [1.515544, 8.993592, 0],
 [1.515544, 11.56319, 0],
 [1.515544, 14.132788, 0],
 [3.031089, -12.847989, 0],
 [3.031089, -10.278391, 0],
 [3.031089, -7.708793, 0],
 [3.031089, -5.139196, 0],
 [3.031089, -2.569598, 0],
 [3.031089, 0.0, 0],
 [3.031089, 2.569598, 0],
 [3.031089, 5.139196, 0],
 [3.031089, 7.708793, 0],
 [3.031089, 10.278391, 0],
 [3.031089, 12.847989, 0],
 [4.546633, -11.56319, 0],
 [4.546633, -8.993592, 0],
 [4.546633, -6.423994, 0],
 [4.546633, -3.854397, 0],
 [4.546633, -1.284799, 0],
 [4.546633, 1.284799, 0],
 [4.546633, 3.854397, 0],
 [4.546633, 6.423994, 0],
 [4.546633, 8.993592, 0],
 [4.546633, 11.56319, 0],
 [6.062178, -10.278391, 0],
 [6.062178, -7.708793, 0],
 [6.062178, -5.139196, 0],
 [6.062178, -2.569598, 0],
 [6.062178, 0.0, 0],
 [6.062178, 2.569598, 0],
 [6.062178, 5.139196, 0],
 [6.062178, 7.708793, 0],
 [6.062178, 10.278391, 0],
 [7.577722, -8.993592, 0],
 [7.577722, -6.423994, 0],
 [7.577722, -3.854397, 0],
 [7.577722, -1.284799, 0],
 [7.577722, 1.284799, 0],
 [7.577722, 3.854397, 0],
 [7.577722, 6.423994, 0],
 [7.577722, 8.993592, 0],
 [9.093267, -7.708793, 0],
 [9.093267, -5.139196, 0],
 [9.093267, -2.569598, 0],
 [9.093267, 0.0, 0],
 [9.093267, 2.569598, 0],
 [9.093267, 5.139196, 0],
 [9.093267, 7.708793, 0],
 [0,0,0]
 ]
)