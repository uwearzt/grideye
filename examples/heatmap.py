#!/usr/bin/env python

import numpy as np
from numpy import genfromtxt

import matplotlib.pyplot as plt

measurements = genfromtxt('measure.csv', delimiter=';')

plt.imshow(measurements)
plt.show()