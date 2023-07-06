import ctypes
import numpy as np
from scipy.sparse import csc_matrix


lib = ctypes.CDLL("./bin/olcamkl.dll")
solve = lib.solve_sparse
float_ptr = ctypes.POINTER(ctypes.c_double)
int_ptr = ctypes.POINTER(ctypes.c_int32)
solve.argtypes = [
    ctypes.c_int32,  # n
    float_ptr,  # a
    int_ptr,  # ia
    int_ptr,  # ja
    float_ptr,  # b
    float_ptr,  # x
]

A = csc_matrix(
    [
        [1.0, -0.5],
        [-1.0, 1.0],
    ]
)
b = np.array([1.0, 0.0], dtype=np.float64)
x = np.zeros(2, dtype=np.float64)

err = solve(
    ctypes.c_int32(A.shape[0]),
    A.data.ctypes.data_as(float_ptr),
    A.indices.ctypes.data_as(int_ptr),
    A.indptr.ctypes.data_as(int_ptr),
    b.ctypes.data_as(float_ptr),
    x.ctypes.data_as(float_ptr),
)

print(f"error = {err}")
print(f"x = {x}")
