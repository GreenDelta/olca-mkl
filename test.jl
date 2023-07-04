const MklInt = Int32

pt = zeros(Int64, 64)
maxfct = Ref{MklInt}(1)
mnum = Ref{MklInt}(1)
mtype = Ref{MklInt}(11)
phase = Ref{MklInt}(13)

n = Ref{MklInt}(2)
a = Vector{Float64}([1.0, -0.5, -1.0, 1.0])
ia = Vector{MklInt}([1, 3, 5])
ja = Vector{MklInt}([1, 2, 1, 2])

perm = zeros(MklInt, 2)
nrhs = Ref{MklInt}(1)
iparm = zeros(MklInt, 64)
msglvl = Ref{MklInt}(0)

b = Vector{Float64}([1.0, 0.0])
x = zeros(Float64, 2)

error = Ref{MklInt}(0)

@ccall "./bin/mkl_rt.2.dll".pardisoinit(
  pt::Ptr{Int64},
  mtype::Ptr{MklInt},
  iparm::Ptr{MklInt})::Cvoid


@ccall "./bin/mkl_rt.2.dll".pardiso(
  pt::Ptr{Int64},
  maxfct::Ptr{MklInt},
  mnum::Ptr{MklInt},
  mtype::Ptr{MklInt},
  phase::Ptr{MklInt},
  n::Ptr{MklInt},
  a::Ptr{Cvoid},
  ia::Ptr{MklInt},
  ja::Ptr{MklInt},
  perm::Ptr{MklInt},
  nrhs::Ptr{MklInt},
  iparm::Ptr{MklInt},
  msglvl::Ptr{MklInt},
  b::Ptr{Cvoid},
  x::Ptr{Cvoid},
  error::Ptr{MklInt}
)::Cvoid
