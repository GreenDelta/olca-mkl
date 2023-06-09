use jni_sys::{jclass, jdoubleArray, jint, JNIEnv};
use libc::c_char;
use std::ptr;

mod blas;

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

    blas::dgemv(
      &trans, &rowsA_64, &colsA_64, &alpha, aPtr, &rowsA_64, xPtr, &inc, &beta,
      yPtr, &inc,
    );

    release_array_f64(env, matrix, aPtr);
    release_array_f64(env, vector, xPtr);
    release_array_f64(env, result, yPtr);
  }
}
