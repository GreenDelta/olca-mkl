use std::ffi::c_void;

use crate::mkl::pardiso;

pub struct Factorization {}

/// Solves a system of linear equations `A*x = b` for a given `A` and `b`.
///
/// The matrix `A` needs to be a square matrix provided in compressed sparse
/// column (CSC) format.
///
/// * `n` - The number of rows and columns of `A`.
/// * `a` - The non-zero values of `A`.
/// * `ia` - The row indices of the non-zero values.
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
    let pt_ptr = pt.as_mut_ptr() as *mut c_void;

    let mut perm = vec![0i32; n as usize];
    let perm_ptr = perm.as_mut_ptr();

    let mut iparm = vec![0i32; 64];
    iparm[0] = 1; // no defaults
    iparm[11] = 2; // CSC format
    iparm[34] = 1; // zero-based indexing
    let iparm_ptr = iparm.as_mut_ptr();

    let mut error = 0;

    let maxfct = 1;
    let mnum = 1;
    let mtype = 11;
    let phase = 13;
    let nrhs = 1;
    let msglvl = 0;

    pardiso(
      pt_ptr, // pt
      &1,     // maxfct
      &1,     // mnum
      &11,    // mtype
      &13,    // phase
      &n, a, ja, ia, perm_ptr, &1, // nrhs
      iparm_ptr, &0, // msglvl
      b, x, &mut error,
    );

    error
  }
}
