# olca-mkl

* https://github.com/doraneko94/pardiso-sys
* https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=caa5ecbe645fcf37fe2dc2538f6437f2ac09325f
* https://www.intel.com/content/www/us/en/docs/onemkl/developer-reference-c/2023-0/pardiso.html

## References

### Official doc:

* Intel: https://www.intel.com/content/www/us/en/docs/onemkl/developer-reference-c/2023-0/pardiso.html#GUID-431916D5-B76D-48A1-ABB5-1A0613FDC0FA

* Old guide: https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=caa5ecbe645fcf37fe2dc2538f6437f2ac09325f

### Linking examples

* Python: https://github.com/haasad/PyPardisoProject
* Java: https://github.com/bytedeco/javacpp-presets/tree/master/mkl
* Rust: https://github.com/doraneko94/pardiso-sys
* C: https://github.com/ww382/CS5220FinalProject/blob/master/pardiso_sym_c.c
* Julia: https://github.com/JuliaSparse/Pardiso.jl

### FFI

* Julia: https://docs.julialang.org/en/v1/manual/calling-c-and-fortran-code/


### AMD

Seems that adding a function `mkl_serv_intel_cpu_true` could be used to call the optimized routines also on AMD:

```c
int mkl_serv_intel_cpu_true() {
  return 1;
}
```

* https://danieldk.eu/mkl-amd-zen/
* https://scrp.econ.cuhk.edu.hk/blog/analysis/2022/02/07/mkl-optimization.html


## TODO

* build tool
  * fetch libs: https://pypi.org/project/mkl/#files
