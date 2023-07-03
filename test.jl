pt = zeros(Int, 64)
maxfct = Ref{Cint}(1)
mnum = Ref{Cint}(1)
mtype = Ref{Cint}(11)
phase = Ref{Cint}(13)
nrhs = Ref{Cint}(1)
msglvl = Ref{Cint}(1)
error = Ref{Cint}(0)

n = Ref{Cint}(5)
ia = Vector{Int32}([1, 4, 6, 9, 12, 14])
ja = Vector{Int32}([1, 2, 4, 1, 2, 3, 4, 5, 1, 3, 4, 2, 5])
a = Vector{Float64}([1.0, -1.0, -3.0, -2.0, 5.0, 4.0, 6.0, 4.0, -4.0, 2.0, 7.0, 8.0, -5.0 ])
b = Vector{Float64}([1.0, 1.0, 1.0, 1.0, 1.0])
x = zeros(Float64, 5)

perm = Int32[]
iparm = zeros(Int32, 64)

@ccall "./bin/mkl_rt.2.dll".pardiso(
  pt::Ptr{Int},
  maxfct::Ptr{Cint},
  mnum::Ptr{Cint},
  mtype::Ptr{Cint},
  phase::Ptr{Cint},
  n::Ptr{Cint},
  a::Ptr{Float64},
  ia::Ptr{Int32},
  ja::Ptr{Int32},
  perm::Ptr{Int32},
  nrhs::Ptr{Cint},
  iparm::Ptr{Int32},
  msglvl::Ptr{Cint},
  b::Ptr{Float64},
  x::Ptr{Float64},
  error::Ptr{Cint}
)::Cvoid
