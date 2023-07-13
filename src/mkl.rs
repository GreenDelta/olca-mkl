use std::ffi::c_char;

#[allow(non_snake_case)]
#[cfg_attr(target_os = "windows", link(name = "mkl_rt.2"))]
#[cfg_attr(target_os = "linux", link(name = "mkl_rt"))]
#[cfg_attr(target_os = "macos", link(name = "mkl_rt"))]
extern "C" {

  /// [DGEMV](http://www.netlib.org/lapack/explore-html/dc/da8/dgemv_8f.html)
  #[cfg_attr(target_os = "windows", link_name = "DGEMV_64")]
  #[cfg_attr(target_os = "linux", link_name = "DGEMV_64")]
  #[cfg_attr(target_os = "macos", link_name = "DGEMV_64")]
  pub fn dgemv(
    TRANS: *const c_char,
    M: *const i64,
    N: *const i64,
    ALPHA: *const f64,
    A: *const f64,
    LDA: *const i64,
    X: *const f64,
    INCX: *const i64,
    BETA: *const f64,
    Y: *mut f64,
    INCY: *const i64,
  );

  /// [DGEMM](http://www.netlib.org/lapack/explore-html/d7/d2b/dgemm_8f.html)
  #[cfg_attr(target_os = "windows", link_name = "DGEMM_64")]
  #[cfg_attr(target_os = "linux", link_name = "DGEMM_64")]
  #[cfg_attr(target_os = "macos", link_name = "DGEMM_64")]
  pub fn dgemm(
    TRANSA: *mut c_char,
    TRANSB: *mut c_char,
    M: *mut i64,
    N: *mut i64,
    K: *mut i64,
    ALPHA: *mut f64,
    A: *mut f64,
    LDA: *mut i64,
    B: *mut f64,
    LDB: *mut i64,
    BETA: *mut f64,
    C: *mut f64,
    LDC: *mut i64,
  );

  /// [DGESV](http://www.netlib.org/lapack/explore-html/d8/d72/dgesv_8f.html)
  #[cfg_attr(target_os = "windows", link_name = "DGESV_64")]
  #[cfg_attr(target_os = "linux", link_name = "DGESV_64")]
  #[cfg_attr(target_os = "macos", link_name = "DGESV_64")]
  pub fn dgesv(
    N: *const i64,
    NRHS: *const i64,
    A: *mut f64,
    LDA: *const i64,
    IPIV: *mut i64,
    B: *mut f64,
    LDB: *const i64,
    INFO: *mut i64,
  );

  /// [DGETRF](http://www.netlib.org/lapack/explore-html/d3/d6a/dgetrf_8f.html)
  #[cfg_attr(target_os = "windows", link_name = "DGETRF_64")]
  #[cfg_attr(target_os = "linux", link_name = "DGETRF_64")]
  #[cfg_attr(target_os = "macos", link_name = "DGETRF_64")]
  pub fn dgetrf(
    M: *const i64,
    N: *const i64,
    A: *mut f64,
    LDA: *const i64,
    IPIV: *mut i64,
    INFO: *mut i64,
  );

  /// [DGETRI](http://www.netlib.org/lapack/explore-html/df/da4/dgetri_8f.html)
  #[cfg_attr(target_os = "windows", link_name = "DGETRI_64")]
  #[cfg_attr(target_os = "linux", link_name = "DGETRI_64")]
  #[cfg_attr(target_os = "macos", link_name = "DGETRI_64")]
  pub fn dgetri(
    N: *mut i64,
    A: *mut f64,
    LDA: *mut i64,
    IPIV: *mut i64,
    WORK: *mut f64,
    LWORK: *mut i64,
    INFO: *mut i64,
  );

  /// [DGETRS](http://www.netlib.org/lapack/explore-html/d6/d49/dgetrs_8f.html)
  #[cfg_attr(target_os = "windows", link_name = "DGETRS_64")]
  #[cfg_attr(target_os = "linux", link_name = "DGETRS_64")]
  #[cfg_attr(target_os = "macos", link_name = "DGETRS_64")]
  pub fn dgetrs(
    TRANS: *const c_char,
    N: *const i64,
    NRHS: *const i64,
    A: *const f64,
    LDA: *const i64,
    IPIV: *const i64,
    B: *mut f64,
    LDB: *const i64,
    INFO: *mut i64,
  );

  /// [PARDISO](https://www.intel.com/content/www/us/en/docs/onemkl/developer-reference-c/2023-0/pardiso.html)
  pub fn pardiso(
    pt: *mut i64,
    maxfct: *const i32,
    mnum: *const i32,
    mtype: *const i32,
    phase: *const i32,
    n: *const i32,
    a: *const f64,
    ia: *const i32,
    ja: *const i32,
    perm: *mut i32,
    nrhs: *const i32,
    iparm: *mut i32,
    msglvl: *const i32,
    b: *mut f64,
    x: *mut f64,
    error: *mut i32,
  );
}
