use jni_sys::{
  jclass, jdoubleArray, jint, jintArray, jlong, jlongArray, JNIEnv,
};
use std::ffi::c_char;

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
pub extern "system" fn Java_org_openlca_mkl_MKL_denseMatrixVectorMultiplication(
  env: *mut JNIEnv,
  _class: jclass,
  rows: jint,
  columns: jint,
  matrix: jdoubleArray,
  vector: jdoubleArray,
  result: jdoubleArray,
) {
  unsafe {
    let aPtr = get_f64(env, matrix);
    let xPtr = get_f64(env, vector);
    let yPtr = get_f64(env, result);

    let trans = 'N' as c_char;
    let alpha: f64 = 1.0;
    let beta: f64 = 0.0;
    let inc: i64 = 1;
    let rowsA_64: i64 = rows as i64;
    let colsA_64: i64 = columns as i64;

    mkl::dgemv(
      &trans, &rowsA_64, &colsA_64, &alpha, aPtr, &rowsA_64, xPtr, &inc, &beta,
      yPtr, &inc,
    );

    drop_f64(env, matrix, aPtr);
    drop_f64(env, vector, xPtr);
    drop_f64(env, result, yPtr);
  }
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
    dense_mvmult(m as i64, n as i64, a_ptr, x_ptr, y_ptr);
    drop_f64(env, a, a_ptr);
    drop_f64(env, x, x_ptr);
    drop_f64(env, y, y_ptr);
  }
}
