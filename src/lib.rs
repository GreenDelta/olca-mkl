use jni_sys::{
  jclass, jdoubleArray, jint, jintArray, jlong, jlongArray, JNIEnv,
};
use std::ffi::c_char;
use std::ptr;

mod mkl;
mod sparse;

pub use crate::sparse::*;

const NULL: *mut u8 = ptr::null_mut();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_version() -> jint {
  1
}

/// Get the raw pointer of the given array from the JVM.
unsafe fn get_array_f64(env: *mut JNIEnv, array: jdoubleArray) -> *mut f64 {
  return (**env).GetDoubleArrayElements.unwrap()(env, array, NULL);
}

/// Give the data behind the raw pointer of the given array back to the JVM.
unsafe fn release_array_f64(
  env: *mut JNIEnv,
  array: jdoubleArray,
  ptr: *mut f64,
) {
  (**env).ReleaseDoubleArrayElements.unwrap()(env, array, ptr, 0);
}

/// Get the raw pointer of the given array from the JVM.
unsafe fn get_array_i32(env: *mut JNIEnv, array: jintArray) -> *mut i32 {
  return (**env).GetIntArrayElements.unwrap()(env, array, NULL);
}

/// Give the data behind the raw pointer of the given array back to the JVM.
unsafe fn release_array_i32(env: *mut JNIEnv, array: jintArray, ptr: *mut i32) {
  (**env).ReleaseIntArrayElements.unwrap()(env, array, ptr, 0);
}

/// Get the raw pointer of the given array from the JVM.
unsafe fn get_array_i64(env: *mut JNIEnv, array: jlongArray) -> *mut i64 {
  return (**env).GetLongArrayElements.unwrap()(env, array, NULL);
}

/// Give the data behind the raw pointer of the given array back to the JVM.
unsafe fn release_array_i64(
  env: *mut JNIEnv,
  array: jlongArray,
  ptr: *mut i64,
) {
  (**env).ReleaseLongArrayElements.unwrap()(env, array, ptr, 0);
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
    let aPtr = get_array_f64(env, matrix);
    let xPtr = get_array_f64(env, vector);
    let yPtr = get_array_f64(env, result);

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

    release_array_f64(env, matrix, aPtr);
    release_array_f64(env, vector, xPtr);
    release_array_f64(env, result, yPtr);
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
    let a_ptr = get_array_f64(env, a);
    let ia_ptr = get_array_i32(env, ia);
    let ja_ptr = get_array_i32(env, ja);
    let b_ptr = get_array_f64(env, b);
    let x_ptr = get_array_f64(env, x);

    let error = solve_sparse(n, a_ptr, ia_ptr, ja_ptr, b_ptr, x_ptr);

    release_array_f64(env, a, a_ptr);
    release_array_i32(env, ia, ia_ptr);
    release_array_i32(env, ja, ja_ptr);
    release_array_f64(env, b, b_ptr);
    release_array_f64(env, x, x_ptr);

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
    let a_ptr = get_array_f64(env, a);
    let ia_ptr = get_array_i32(env, ia);
    let ja_ptr = get_array_i32(env, ja);
    let ptr_ptr = get_array_i64(env, ptr);

    let error = sparse_factorization(n, a_ptr, ia_ptr, ja_ptr, ptr_ptr);

    release_array_f64(env, a, a_ptr);
    release_array_i32(env, ia, ia_ptr);
    release_array_i32(env, ja, ja_ptr);
    release_array_i64(env, ptr, ptr_ptr);

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
    let b_ptr = get_array_f64(env, b);
    let x_ptr = get_array_f64(env, x);

    let error = solve_sparse_factorization(ptr, b_ptr, x_ptr);

    release_array_f64(env, b, b_ptr);
    release_array_f64(env, x, x_ptr);

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
