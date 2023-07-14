# olca-mkl

This is an experimental project for using the Intel Math Kernel as calulcation library in openLCA.

## AMD

It seems that adding a function `mkl_serv_intel_cpu_true` could be used to call the optimized routines also on AMD:

```c
int mkl_serv_intel_cpu_true() {
  return 1;
}
```

* https://danieldk.eu/mkl-amd-zen/
* https://scrp.econ.cuhk.edu.hk/blog/analysis/2022/02/07/mkl-optimization.html
