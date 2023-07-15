use jni_sys::{
  jclass, jdoubleArray, jint, jintArray, jlong, jlongArray, JNIEnv,
};

mod arrays;
mod dense;
mod mkl;
mod sparse;

use crate::arrays::*;
use crate::dense::*;
use crate::sparse::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_version() -> jint {
  1
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_solveSparse(
  env: *mut JNIEnv,
  _class: jclass,
  n: jint,
  a: jdoubleArray,
  ia: jintArray,
  ja: jintArray,
  b: jdoubleArray,
  x: jdoubleArray,
) -> jint {
  unsafe {
    let a_ptr = get_f64(env, a);
    let ia_ptr = get_i32(env, ia);
    let ja_ptr = get_i32(env, ja);
    let b_ptr = get_f64(env, b);
    let x_ptr = get_f64(env, x);

    let error = solve_sparse(n, a_ptr, ia_ptr, ja_ptr, b_ptr, x_ptr);

    drop_f64(env, a, a_ptr);
    drop_i32(env, ia, ia_ptr);
    drop_i32(env, ja, ja_ptr);
    drop_f64(env, b, b_ptr);
    drop_f64(env, x, x_ptr);

    error as jint
  }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_sparseFactorization(
  env: *mut JNIEnv,
  _class: jclass,
  n: jint,
  a: jdoubleArray,
  ia: jintArray,
  ja: jintArray,
  ptr: jlongArray,
) -> jint {
  unsafe {
    let a_ptr = get_f64(env, a);
    let ia_ptr = get_i32(env, ia);
    let ja_ptr = get_i32(env, ja);
    let ptr_ptr = get_i64(env, ptr);

    let error = sparse_factorization(n, a_ptr, ia_ptr, ja_ptr, ptr_ptr);

    drop_f64(env, a, a_ptr);
    drop_i32(env, ia, ia_ptr);
    drop_i32(env, ja, ja_ptr);
    drop_i64(env, ptr, ptr_ptr);

    error
  }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_solveSparseFactorization(
  env: *mut JNIEnv,
  _class: jclass,
  ptr: jlong,
  b: jdoubleArray,
  x: jdoubleArray,
) -> jint {
  unsafe {
    let b_ptr = get_f64(env, b);
    let x_ptr = get_f64(env, x);

    let error = solve_sparse_factorization(ptr, b_ptr, x_ptr);

    drop_f64(env, b, b_ptr);
    drop_f64(env, x, x_ptr);

    error as jint
  }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_disposeSparseFactorization(
  _env: *mut JNIEnv,
  _class: jclass,
  ptr: jlong,
) {
  dispose_sparse_factorization(ptr);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_denseFactorization(
  env: *mut JNIEnv,
  _class: jclass,
  n: jint,
  a: jdoubleArray,
  ptr: jlongArray,
) -> jint {
  unsafe {
    let a_ptr = get_f64(env, a);
    let ptr_ptr = get_i64(env, ptr);
    let info = dense_factorization(n as i64, a_ptr, ptr_ptr);
    drop_f64(env, a, a_ptr);
    drop_i64(env, ptr, ptr_ptr);
    info as jint
  }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_solveDenseFactorization(
  env: *mut JNIEnv,
  _class: jclass,
  ptr: jlong,
  nrhs: jint,
  b: jdoubleArray,
) -> jint {
  unsafe {
    let b_ptr = get_f64(env, b);
    let info = solve_dense_factorization(ptr, nrhs as i64, b_ptr);
    drop_f64(env, b, b_ptr);
    info as jint
  }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_disposeDenseFactorization(
  _env: *mut JNIEnv,
  _class: jclass,
  ptr: jlong,
) {
  dispose_dense_factorization(ptr);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_denseMatrixVectorMul(
  env: *mut JNIEnv,
  _class: jclass,
  m: jint,
  n: jint,
  a: jdoubleArray,
  x: jdoubleArray,
  y: jdoubleArray,
) {
  unsafe {
    let a_ptr = get_f64(env, a);
    let x_ptr = get_f64(env, x);
    let y_ptr = get_f64(env, y);
    dense_mvmul(m as i64, n as i64, a_ptr, x_ptr, y_ptr);
    drop_f64(env, a, a_ptr);
    drop_f64(env, x, x_ptr);
    drop_f64(env, y, y_ptr);
  }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_denseMatrixMul(
  env: *mut JNIEnv,
  _class: jclass,
  m: jint,
  n: jint,
  k: jint,
  a: jdoubleArray,
  b: jdoubleArray,
  c: jdoubleArray,
) {
  unsafe {
    let a_ptr = get_f64(env, a);
    let b_ptr = get_f64(env, b);
    let c_ptr = get_f64(env, c);
    dense_mmul(m as i64, n as i64, k as i64, a_ptr, b_ptr, c_ptr);
    drop_f64(env, a, a_ptr);
    drop_f64(env, b, b_ptr);
    drop_f64(env, c, c_ptr);
  }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_solveDense(
  env: *mut JNIEnv,
  _class: jclass,
  n: jint,
  nrhs: jint,
  a: jdoubleArray,
  b: jdoubleArray,
) -> jint {
  unsafe {
    let a_ptr = get_f64(env, a);
    let b_ptr = get_f64(env, b);
    let info = solve_dense(n as i64, nrhs as i64, a_ptr, b_ptr);
    drop_f64(env, a, a_ptr);
    drop_f64(env, b, b_ptr);
    info as jint
  }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_invertDense(
  env: *mut JNIEnv,
  _class: jclass,
  n: jint,
  a: jdoubleArray,
) -> jint {
  unsafe {
    let a_ptr = get_f64(env, a);
    let info = invert_dense(n as i64, a_ptr);
    drop_f64(env, a, a_ptr);
    info as jint
  }
}
