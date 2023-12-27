To find the axis of symmetry of a non-square matrix of 1's and 0's, you can follow these steps:

1. **Calculate the center of mass**: The center of mass is the average position of all the 1's in the matrix. For a 2D matrix, you can calculate the center of mass (x, y) as follows:

    $$x = \frac{\sum_{i=1}^{n} x_i}{n}$$
    $$y = \frac{\sum_{i=1}^{n} y_i}{n}$$

    where $(x_i, y_i)$ are the coordinates of the 1's in the matrix and $n$ is the total number of 1's.

2. **Calculate the moments of inertia**: The moments of inertia are measures of the matrix's resistance to rotation about the axes. For a 2D matrix, you can calculate the moments of inertia $I_x$ and $I_y$ as follows:

    $$I_x = \sum_{i=1}^{n} (y_i - y)^2$$
    $$I_y = \sum_{i=1}^{n} (x_i - x)^2$$

    where $(x_i, y_i)$ are the coordinates of the 1's in the matrix, $(x, y)$ is the center of mass, and $n$ is the total number of 1's.

3. **Find the axis of symmetry**: The axis of symmetry is the axis about which the matrix is most resistant to rotation. This is the axis corresponding to the larger moment of inertia. If $I_x > I_y$, the axis of symmetry is the x-axis, and if $I_y > I_x$, the axis of symmetry is the y-axis.

Please note that this method assumes that the matrix is a representation of a 2D object with uniform density, and that the 1's in the matrix represent the object's mass. If the matrix does not meet these assumptions, the method may not accurately find the axis of symmetry. Also, this method finds an approximate axis of symmetry and may not be accurate for matrices with complex shapes or multiple axes of symmetry. For such matrices, more advanced techniques may be required. 

I hope this helps! If you have any other questions, feel free to ask. 😊