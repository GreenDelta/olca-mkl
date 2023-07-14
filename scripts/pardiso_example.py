# see also https://github.com/haasad/PyPardisoProject

import ctypes
import numpy as np

lib = ctypes.CDLL("./bin/mkl_rt.2.dll")
# lib = ctypes.CDLL("./bin/libmkl_rt.so.2")
pardiso = lib.pardiso
pardiso.argtypes = [
    ctypes.POINTER(ctypes.c_int64),  # pt
    ctypes.POINTER(ctypes.c_int32),  # maxfct
    ctypes.POINTER(ctypes.c_int32),  # mnum
    ctypes.POINTER(ctypes.c_int32),  # mtype
    ctypes.POINTER(ctypes.c_int32),  # phase
    ctypes.POINTER(ctypes.c_int32),  # n
    ctypes.POINTER(ctypes.c_void_p),  # a
    ctypes.POINTER(ctypes.c_int32),  # ia
    ctypes.POINTER(ctypes.c_int32),  # ja
    ctypes.POINTER(ctypes.c_int32),  # perm
    ctypes.POINTER(ctypes.c_int32),  # nrhs
    ctypes.POINTER(ctypes.c_int32),  # iparm
    ctypes.POINTER(ctypes.c_int32),  # msglvl
    ctypes.POINTER(ctypes.c_void_p),  # b
    ctypes.POINTER(ctypes.c_void_p),  # x
    ctypes.POINTER(ctypes.c_int32),  # error
]

mtype = 11
phase = 13
msglvl = 0
pt = np.zeros(64, dtype=np.int64)
iparm = np.zeros(64, dtype=np.int32)
perm = np.zeros(0, dtype=np.int32)

n = 2
a = np.array([1.0, -0.5, -1.0, 1.0], dtype=np.float64)
ia = np.array([1, 3, 5], dtype=np.int32)
ja = np.array([1, 2, 1, 2], dtype=np.int32)
b = np.array([1.0, 0.0], dtype=np.float64)
x = np.array([0.0, 0.0], dtype=np.float64)

# n = 1
# a = np.array([1.0], dtype=np.float64)
# ia = np.array([1, 2], dtype=np.int32)
# ja = np.array([1], dtype=np.int32)
# b = np.array([1.0], dtype=np.float64)
# x = np.array([0.0], dtype=np.float64)

c_int32_p = ctypes.POINTER(ctypes.c_int32)
c_float64_p = ctypes.POINTER(ctypes.c_void_p)

error = 0

pardiso(
    pt.ctypes.data_as(ctypes.POINTER(ctypes.c_int64)),  # pt
    ctypes.byref(ctypes.c_int32(1)),  # maxfct
    ctypes.byref(ctypes.c_int32(1)),  # mnum
    ctypes.byref(ctypes.c_int32(mtype)),  # mtype -> 11 for real-nonsymetric
    ctypes.byref(ctypes.c_int32(phase)),  # phase -> 13
    ctypes.byref(ctypes.c_int32(n)),  # N -> number of equations/size of matrix
    a.ctypes.data_as(c_float64_p),  # A -> non-zero entries in matrix
    ia.ctypes.data_as(c_int32_p),  # ia -> csr-indptr
    ja.ctypes.data_as(c_int32_p),  # ja -> csr-indices
    perm.ctypes.data_as(c_int32_p),  # perm -> empty
    ctypes.byref(ctypes.c_int32(1)),  # nrhs
    iparm.ctypes.data_as(c_int32_p),  # iparm-array
    ctypes.byref(
        ctypes.c_int32(msglvl)
    ),  # msg-level -> 1: statistical info is printed
    b.ctypes.data_as(c_float64_p),  # b -> right-hand side vector/matrix
    x.ctypes.data_as(c_float64_p),  # x -> output
    ctypes.byref(ctypes.c_int32(error)),  # pardiso error
)

print(x)
