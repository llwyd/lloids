import matplotlib.pyplot as plt
import numpy as np


x_lim = 2
y_lim = 2

given_bird = [-1, 1]

fig = plt.figure()
ax = fig.add_subplot(1,1,1)

# https://stackoverflow.com/questions/31556446/how-to-draw-axis-in-the-middle-of-the-figure
ax.spines['left'].set_position('zero')
ax.spines['bottom'].set_position('zero')

ax.spines['right'].set_color('none')
ax.spines['top'].set_color('none')

plt.scatter(-1,1, color='black')
plt.xlim(-x_lim, x_lim)
plt.ylim(-y_lim, y_lim)
plt.show()
