use std::ffi::c_char;
use std::ptr;

use crate::mkl::*;

struct Factorization {
  n: i64,
  data: Vec<f64>,
  pivot: Vec<i64>,
}

impl Factorization {
  fn of(n: i64, matrix: *const f64) -> (Option<Factorization>, i64) {
    println!("debug: create dense factorization");

    let size = (n * n) as usize;
    let mut data = vec![0f64; size];
    let data_ptr = data.as_mut_ptr();
    let mut pivot = vec![0i64; n as usize];
    let mut info = 0i64;

    unsafe {
      ptr::copy_nonoverlapping(matrix, data_ptr, size);
      dgetrf(
        &n,                 // M
        &n,                 // N
        data_ptr,           // A
        &n,                 // LDA
        pivot.as_mut_ptr(), // IPIV
        &mut info,          // INFO
      );
    }

    if info == 0 {
      (Some(Factorization { n, data, pivot }), 0)
    } else {
      (None, info)
    }
  }

  fn solve(&self, nrhs: i64, b: *mut f64) -> i64 {
    println!("debug: solve dense factorization");
    let mut info = 0i64;
    unsafe {
      dgetrs(
        &('N' as c_char),    // TRANS
        &self.n,             // N
        &nrhs,               // NRHS
        self.data.as_ptr(),  // A
        &self.n,             // LDA
        self.pivot.as_ptr(), // IPIV
        b,                   // B
        &self.n,             // LDB
        &mut info,           // INFO
      )
    }
    info
  }
}

impl Drop for Factorization {
  fn drop(&mut self) {
    println!("debug: dispose dense factorization");
  }
}

#[inline]
#[no_mangle]
pub extern "system" fn dense_factorization(
  n: i64,
  a: *const f64,
  ptr: *mut i64,
) -> i64 {
  let (opt, info) = Factorization::of(n, a);
  if opt.is_some() {
    unsafe {
      let fact = opt.unwrap();
      let fact_ptr = Box::into_raw(Box::new(fact));
      *ptr = fact_ptr as i64;
    }
  }
  info
}

#[inline]
#[no_mangle]
pub extern "system" fn solve_dense_factorization(
  fact_ptr: i64,
  nrhs: i64,
  b: *mut f64,
) -> i64 {
  unsafe {
    let fact = fact_ptr as *const Factorization;
    (*fact).solve(nrhs, b)
  }
}

#[inline]
#[no_mangle]
pub extern "system" fn dispose_dense_factorization(fact_ptr: i64) {
  unsafe {
    let p = fact_ptr as *mut Factorization;
    let factorization = Box::from_raw(p);
    drop(factorization);
  }
}

#[inline]
#[no_mangle]
pub extern "system" fn dense_mvmul(
  m: i64,
  n: i64,
  a: *const f64,
  x: *const f64,
  y: *mut f64,
) {
  unsafe {
    dgemv(
      &('N' as c_char), // TRANS
      &m,               // M
      &n,               // N
      &1.0,             // ALPHA
      a,                // A
      &m,               // LDA
      x,                // X
      &1,               // INCX
      &0.0,             // BETA
      y,                // Y
      &1,               // INCY
    );
  }
}

#[inline]
#[no_mangle]
pub extern "system" fn dense_mmul(
  m: i64,
  n: i64,
  k: i64,
  a: *const f64,
  b: *const f64,
  c: *mut f64,
) {
  let trans = 'N' as c_char;
  unsafe {
    dgemm(
      &trans, // TRANSA
      &trans, // TRANSB
      &m,     // M
      &n,     // N
      &k,     // K
      &1f64,  // ALPHA
      a,      // A
      &m,     // LDA
      b,      // B
      &k,     // LDB
      &0f64,  // BETA
      c,      // C
      &m,     // LDC
    )
  }
}

#[inline]
#[no_mangle]
pub extern "system" fn solve_dense(
  n: i64,
  nrhs: i64,
  a: *mut f64,
  b: *mut f64,
) -> i64 {
  let mut info = 0i64;
  let mut ipiv = vec![0i64; n as usize];
  unsafe {
    dgesv(
      &n,                // N
      &nrhs,             // NRHS
      a,                 // A
      &n,                // LDA
      ipiv.as_mut_ptr(), // IPIV
      b,                 // B
      &n,                // LDB
      &mut info,         // INFO
    );
  }
  info
}

#[inline]
#[no_mangle]
pub extern "system" fn invert_dense(n: i64, a: *mut f64) -> i64 {
  // factorization
  let mut pivot = vec![0i64; n as usize];
  let ipiv = pivot.as_mut_ptr();
  let mut info = 0i64;
  unsafe {
    dgetrf(
      &n,        // M
      &n,        // N
      a,         // A
      &n,        // LDA
      ipiv,      // IPIV
      &mut info, // INFO
    );
  }
  if info != 0 {
    return info;
  }

  let lwork = 64 * 2 * n;
  let mut work = vec![0f64; lwork as usize];
  let work_ptr = work.as_mut_ptr();

  // inversion
  unsafe {
    dgetri(
      &n,        // N
      a,         // A
      &n,        // LDA
      ipiv,      // IPIV
      work_ptr,  // WORK
      &lwork,    // LWORK
      &mut info, // INFO
    );
  }
  info
}
