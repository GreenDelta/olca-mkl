// The methods of this module call the pardiso function. See the documentation
// of that function for details:
// https://www.intel.com/content/www/us/en/docs/onemkl/developer-reference-c/2023-0/pardiso.html

use std::ptr;

use crate::mkl::pardiso;

// Analysis, numerical factorization, solve, iterative refinement
const PHASE_ALL: i32 = 13;

// Analysis, numerical factorization
const PHASE_FACT: i32 = 12;

// Solve, iterative refinement
const PHASE_SOLVE: i32 = 33;

// Release all internal memory for all matrices
const PHASE_CLEANUP: i32 = -1;

struct Solver {
  n: i32,
  pt: Vec<i64>,
  perm: Vec<i32>,
  iparm: Vec<i32>,
}

impl Solver {
  fn new(n: i32) -> Self {
    println!("debug: create solver");
    let pt = vec![0i64; 64];
    let perm = vec![0i32; n as usize];
    let mut iparm = vec![0i32; 64];
    iparm[0] = 1; // no defaults
    iparm[1] = 2;
    iparm[9] = 13;
    iparm[10] = 1;
    iparm[11] = 2; // CSC format
    iparm[12] = 1;
    iparm[34] = 1; // zero-based indexing
    Solver { n, pt, perm, iparm }
  }

  fn solve(
    &mut self,
    a: *const f64,
    ia: *const i32,
    ja: *const i32,
    b: *mut f64,
    x: *mut f64,
  ) -> i32 {
    println!("debug: solve Ax=b for x");
    let mut error = 0;
    unsafe {
      pardiso(
        self.pt.as_mut_ptr(),    // pt
        &1,                      // maxfct
        &1,                      // mnum
        &11,                     // mtype
        &PHASE_ALL,              // phase
        &self.n,                 // n
        a,                       // a
        ja,                      // ja
        ia,                      // ia
        self.perm.as_mut_ptr(),  // perm
        &1,                      // nrhs
        self.iparm.as_mut_ptr(), // iparm
        &0,                      // msglvl
        b,                       // b
        x,                       // x
        &mut error,              // error
      );
    }
    error
  }

  fn factorize(
    &mut self,
    a: *const f64,
    ia: *const i32,
    ja: *const i32,
  ) -> i32 {
    println!("debug: factorize matrix");
    let mut error = 0;
    unsafe {
      pardiso(
        self.pt.as_mut_ptr(),    // pt
        &1,                      // maxfct
        &1,                      // mnum
        &11,                     // mtype
        &PHASE_FACT,             // phase
        &self.n,                 // n
        a,                       // a
        ja,                      // ja
        ia,                      // ia
        self.perm.as_mut_ptr(),  // perm
        &1,                      // nrhs
        self.iparm.as_mut_ptr(), // iparm
        &0,                      // msglvl
        ptr::null_mut(),         // b
        ptr::null_mut(),         // x
        &mut error,              // error
      );
    }
    error
  }

  fn solve_with_factorization(&mut self, b: *mut f64, x: *mut f64) -> i32 {
    println!("debug: solve with factorization");
    let mut error = 0;
    unsafe {
      pardiso(
        self.pt.as_mut_ptr(),    // pt
        &1,                      // maxfct
        &1,                      // mnum
        &11,                     // mtype
        &PHASE_SOLVE,            // phase
        &self.n,                 // n
        ptr::null(),             // a
        ptr::null(),             // ja
        ptr::null(),             // ia
        self.perm.as_mut_ptr(),  // perm
        &1,                      // nrhs
        self.iparm.as_mut_ptr(), // iparm
        &0,                      // msglvl
        b,                       // b
        x,                       // x
        &mut error,              // error
      );
    }
    error
  }
}

impl Drop for Solver {
  fn drop(&mut self) {
    println!("debug: drop solver");
    unsafe {
      let mut error = 0i32;

      pardiso(
        self.pt.as_mut_ptr(),    // pt
        &1,                      // maxfct
        &1,                      // mnum
        &11,                     // mtype
        &PHASE_CLEANUP,          // phase
        &self.n,                 // n
        ptr::null(),             // a
        ptr::null(),             // ja
        ptr::null(),             // ia
        self.perm.as_mut_ptr(),  // perm
        &1,                      // nrhs
        self.iparm.as_mut_ptr(), // iparm
        &0,                      // msglvl
        ptr::null_mut(),         // b
        ptr::null_mut(),         // x
        &mut error,              // error
      );

      if error != 0 {
        println!("error: mkl::pardiso failed to cleanup");
      }
    }
  }
}

#[no_mangle]
pub extern "system" fn sparse_factorization(
  n: i32,
  a: *const f64,
  ia: *const i32,
  ja: *const i32,
  ptr: *mut i64,
) -> i32 {
  let mut solver = Solver::new(n);
  let err = solver.factorize(a, ia, ja);
  if err != 0 {
    drop(solver);
    return err;
  }
  unsafe {
    let solver_ptr = Box::into_raw(Box::new(solver));
    *ptr = solver_ptr as i64;
  }
  0
}

#[no_mangle]
pub extern "system" fn solve_sparse_factorization(
  solver_ptr: i64,
  b: *mut f64,
  x: *mut f64,
) -> i32 {
  unsafe {
    let solver = solver_ptr as *mut Solver;
    (*solver).solve_with_factorization(b, x)
  }
}

#[no_mangle]
pub extern "system" fn dispose_sparse_factorization(
  solver_ptr: i64,
) {
  unsafe {
    let p = solver_ptr as *mut Solver;
    let solver = Box::from_raw(p);
    drop(solver);
  }
}


/// Solves `A*x = b` where A is provided in CSC format.
///
/// * `n` - The number of rows and columns of `A`.
/// * `a` - The non-zero values of `A`.
/// * `ia` - The row indices of the non-zero values of A.
/// * `ja` - The column pointers of `A`.
/// * `b` - The right-hand side vector of size `n`.
/// * `x` - The solution vector of size `n`.
#[no_mangle]
pub extern "system" fn solve_sparse(
  n: i32,
  a: *const f64,
  ia: *const i32,
  ja: *const i32,
  b: *mut f64,
  x: *mut f64,
) -> i32 {
  unsafe {
    let mut pt = vec![0i64; 64];
    let pt_ptr = pt.as_mut_ptr();

    let mut perm = vec![0i32; n as usize];
    let perm_ptr = perm.as_mut_ptr();

    let mut iparm = vec![0i32; 64];
    iparm[0] = 1; // no defaults
    iparm[1] = 2;
    iparm[9] = 13;
    iparm[10] = 1;
    iparm[11] = 2; // CSC format
    iparm[12] = 1;
    iparm[34] = 1; // zero-based indexing
    let iparm_ptr = iparm.as_mut_ptr();

    let mut error = 0;

    pardiso(
      pt_ptr,     // pt
      &1,         // maxfct
      &1,         // mnum
      &11,        // mtype
      &13,        // phase
      &n,         //
      a,          //
      ja,         //
      ia,         //
      perm_ptr,   //
      &1,         // nrhs
      iparm_ptr,  //
      &0,         // msglvl
      b,          //
      x,          //
      &mut error, //
    );

    let cleanup_err = cleanup(pt_ptr, perm_ptr, iparm_ptr);

    if error != 0 {
      error
    } else {
      cleanup_err
    }
  }
}

fn cleanup(pt: *mut i64, perm: *mut i32, iparm: *mut i32) -> i32 {
  unsafe {
    let a: *const f64 = ptr::null();
    let ja: *const i32 = ptr::null();
    let ia: *const i32 = ptr::null();
    let b: *mut f64 = ptr::null_mut();
    let x: *mut f64 = ptr::null_mut();
    let mut error = 0i32;

    pardiso(
      pt,         // pt
      &1,         // maxfct
      &1,         // mnum
      &11,        // mtype
      &-1,        // phase
      &0,         //
      a,          //
      ja,         //
      ia,         //
      perm,       //
      &1,         // nrhs
      iparm,      //
      &0,         // msglvl
      b,          //
      x,          //
      &mut error, //
    );

    error
  }
}
