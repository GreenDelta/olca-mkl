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
      dgetrf(&n, &n, data_ptr, &n, pivot.as_mut_ptr(), &mut info);
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
        &('N' as c_char),
        &self.n,
        &nrhs,
        self.data.as_ptr(),
        &self.n,
        self.pivot.as_ptr(),
        b,
        &self.n,
        &mut info,
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

#[no_mangle]
pub extern "system" fn dispose_dense_factorization(
  fact_ptr: i64) {
  unsafe {
    let p = fact_ptr as *mut Factorization;
    let factorization = Box::from_raw(p);
    drop(factorization);
  }
}
