const MklInt = Int64

pt = zeros(Int64, 64)
maxfct = Ref{MklInt}(1)
mnum = Ref{MklInt}(1)
mtype = Ref{MklInt}(11)
phase = Ref{MklInt}(13)
nrhs = Ref{MklInt}(1)
msglvl = Ref{MklInt}(1)
error = Ref{MklInt}(0)

n = Ref{MklInt}(5)
ia = Vector{MklInt}([1, 4, 6, 9, 12, 14])
ja = Vector{MklInt}([1, 2, 4, 1, 2, 3, 4, 5, 1, 3, 4, 2, 5])
a = Vector{Float64}([1.0, -1.0, -3.0, -2.0, 5.0, 4.0, 6.0, 4.0, -4.0, 2.0, 7.0, 8.0, -5.0 ])
b = Vector{Float64}([1.0, 1.0, 1.0, 1.0, 1.0])
x = zeros(Float64, 5)

perm = MklInt[]
iparm = zeros(MklInt, 64)

@ccall "./bin/mkl_rt.2.dll".pardiso(
  pt::Ptr{Int64},
  maxfct::Ptr{MklInt},
  mnum::Ptr{MklInt},
  mtype::Ptr{MklInt},
  phase::Ptr{MklInt},
  n::Ptr{MklInt},
  a::Ptr{Float64},
  ia::Ptr{MklInt},
  ja::Ptr{MklInt},
  perm::Ptr{MklInt},
  nrhs::Ptr{MklInt},
  iparm::Ptr{MklInt},
  msglvl::Ptr{MklInt},
  b::Ptr{Float64},
  x::Ptr{Float64},
  error::Ptr{MklInt}
)::Cvoid
