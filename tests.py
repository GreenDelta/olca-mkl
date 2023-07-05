import ctypes
from scipy.sparse import csc_matrix

lib = ctypes.CDLL("./bin/olcamkl.dll")
solve = lib.solve
solve.argtypes = [

]
solve(

)
